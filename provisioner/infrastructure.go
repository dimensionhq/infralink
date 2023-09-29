package main

import (
	"context"
	"fmt"
	"os"

	"github.com/apenella/go-ansible/pkg/options"
	"github.com/apenella/go-ansible/pkg/playbook"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ecr"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

type Common struct {
	name     string
	key      string
	user     string
	ami      string
	instance string
	verbose  bool
}

type Node struct {
	role       string
	ip         string
	kubeconfig string
}

func upsertInitialStack(ctx *pulumi.Context, common Common) error {
	bucket, err := s3.NewBucket(ctx, common.name, &s3.BucketArgs{
		Acl: pulumi.String("private"),
	})
	if err != nil {
		return err
	}

	ctx.Export("bucket", bucket.Bucket)

	return nil
}

func upsertSecondaryStack(ctx *pulumi.Context, common Common, master Node, worker Node) error {
	zones, err := aws.GetAvailabilityZones(ctx, nil)
	if err != nil {
		return err
	}

	vpc, err := ec2.NewVpc(ctx, "vpc", &ec2.VpcArgs{
		CidrBlock: pulumi.StringPtr("172.0.0.0/16"),
	})
	if err != nil {
		return err
	}

	gateway, err := ec2.NewInternetGateway(ctx, "gateway", &ec2.InternetGatewayArgs{
		VpcId: vpc.ID(),
	})
	if err != nil {
		return err
	}

	securityGroup, err := ec2.NewSecurityGroup(ctx, "security-group", &ec2.SecurityGroupArgs{
		Ingress: ec2.SecurityGroupIngressArray{
			&ec2.SecurityGroupIngressArgs{
				FromPort: pulumi.Int(0),
				ToPort:   pulumi.Int(0),
				Protocol: pulumi.String("-1"),
				CidrBlocks: pulumi.StringArray{
					vpc.CidrBlock,
				},
			},
			&ec2.SecurityGroupIngressArgs{
				FromPort: pulumi.Int(22),
				ToPort:   pulumi.Int(22),
				Protocol: pulumi.String("TCP"),
				CidrBlocks: pulumi.StringArray{
					pulumi.String("0.0.0.0/0"),
				},
			},
		},
		Egress: ec2.SecurityGroupEgressArray{
			&ec2.SecurityGroupEgressArgs{
				FromPort: pulumi.Int(0),
				ToPort:   pulumi.Int(0),
				Protocol: pulumi.String("-1"),
				CidrBlocks: pulumi.StringArray{
					pulumi.String("0.0.0.0/0"),
				},
			},
		},
		VpcId: vpc.ID(),
	})
	if err != nil {
		return err
	}

	key, err := os.ReadFile(common.key)
	if err != nil {
		return err
	}

	keypair, err := ec2.NewKeyPair(ctx, "keypair", &ec2.KeyPairArgs{
		KeyName:   pulumi.String(common.name), //TODO - should be named after the user/service
		PublicKey: pulumi.String(key),
	})
	if err != nil {
		return err
	}

	repository, err := ecr.NewRepository(ctx, "repository", &ecr.RepositoryArgs{
		Name: pulumi.String(common.name),
	})
	if err != nil {
		return err
	}

	ctx.Export("repository", repository.RepositoryUrl)

	token, err := ecr.GetAuthorizationToken(ctx, nil, nil)
	if err != nil {
		return err
	}

	ctx.Export("username", pulumi.String(token.UserName))
	ctx.Export("password", pulumi.String(token.Password))
	ctx.Export("token", pulumi.String(token.AuthorizationToken))
	ctx.Export("id", pulumi.String(token.Id))

	//Master setup
	//TODO - move to a common func
	subnetMaster, err := ec2.NewSubnet(ctx, "subnet-master", &ec2.SubnetArgs{
		AvailabilityZone:    pulumi.StringPtr(zones.Names[0]),
		VpcId:               vpc.ID(),
		CidrBlock:           pulumi.StringPtr("172.0.1.0/24"),
		MapPublicIpOnLaunch: pulumi.Bool(true),
	})
	if err != nil {
		return err
	}

	routeTableMaster, err := ec2.NewRouteTable(ctx, "route-table-master", &ec2.RouteTableArgs{
		VpcId: vpc.ID(),
		Routes: ec2.RouteTableRouteArray{
			&ec2.RouteTableRouteArgs{
				CidrBlock: pulumi.String("0.0.0.0/0"),
				GatewayId: gateway.ID(),
			},
		},
	})

	_, err = ec2.NewRouteTableAssociation(ctx, "route-table-association-master", &ec2.RouteTableAssociationArgs{
		RouteTableId: routeTableMaster.ID(),
		SubnetId:     subnetMaster.ID(),
	})

	masterInstance, err := ec2.NewInstance(ctx, "instanceType-master", &ec2.InstanceArgs{
		KeyName:      keypair.KeyName,
		Ami:          pulumi.String(common.ami),
		InstanceType: pulumi.String(common.instance),
		Tags: pulumi.StringMap{
			"Name": pulumi.String(master.role),
		},
		SubnetId: subnetMaster.ID(),
		VpcSecurityGroupIds: pulumi.StringArray{
			securityGroup.ID(),
		},
	})
	if err != nil {
		return err
	}

	ctx.Export("master-ip", masterInstance.PublicIp)

	//Worker setup
	//TODO - move to a common func
	subnetWorker, err := ec2.NewSubnet(ctx, "subnet-worker", &ec2.SubnetArgs{
		AvailabilityZone:    pulumi.StringPtr(zones.Names[1]),
		VpcId:               vpc.ID(),
		CidrBlock:           pulumi.StringPtr("172.0.2.0/24"),
		MapPublicIpOnLaunch: pulumi.Bool(true),
	})
	if err != nil {
		return err
	}

	routeTableWorker, err := ec2.NewRouteTable(ctx, "route-table-worker", &ec2.RouteTableArgs{
		VpcId: vpc.ID(),
		Routes: ec2.RouteTableRouteArray{
			&ec2.RouteTableRouteArgs{
				CidrBlock: pulumi.String("0.0.0.0/0"),
				GatewayId: gateway.ID(),
			},
		},
	})

	_, err = ec2.NewRouteTableAssociation(ctx, "route-table-association-worker", &ec2.RouteTableAssociationArgs{
		RouteTableId: routeTableWorker.ID(),
		SubnetId:     subnetWorker.ID(),
	})

	workerInstance, err := ec2.NewInstance(ctx, "instanceType-worker", &ec2.InstanceArgs{
		KeyName:      keypair.KeyName,
		Ami:          pulumi.String(common.ami),
		InstanceType: pulumi.String(common.instance),
		Tags: pulumi.StringMap{
			"Name": pulumi.String(worker.role),
		},
		SubnetId: subnetWorker.ID(),
		VpcSecurityGroupIds: pulumi.StringArray{
			securityGroup.ID(),
		},
	})
	if err != nil {
		return err
	}

	ctx.Export("worker-ip", workerInstance.PublicIp)

	return nil
}

func (n *Node) setupK0s(ctx context.Context, common Common) error {
	ansiblePlaybookConnectionOptions := &options.AnsibleConnectionOptions{
		User: common.user,
	}

	ansiblePlaybookOptions := &playbook.AnsiblePlaybookOptions{
		Inventory: fmt.Sprintf("%s,", n.ip), //That comma is required
	}

	privilegeEscalationOptions := &options.AnsiblePrivilegeEscalationOptions{
		Become: true,
	}

	playbookCmd := &playbook.AnsiblePlaybookCmd{
		Playbooks: []string{
			"assets/playbooks/install.yaml",
			fmt.Sprintf("assets/playbooks/%s.yaml", n.role),
		},
		ConnectionOptions:          ansiblePlaybookConnectionOptions,
		Options:                    ansiblePlaybookOptions,
		PrivilegeEscalationOptions: privilegeEscalationOptions,
	}
	err := playbookCmd.Run(ctx)
	if err != nil {
		return err
	}

	return err
}
