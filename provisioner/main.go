package main

import (
	"context"
	"fmt"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws"
	"log"
	"os"

	"github.com/apenella/go-ansible/pkg/options"
	"github.com/apenella/go-ansible/pkg/playbook"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/auto"
	"github.com/pulumi/pulumi/sdk/v3/go/auto/optup"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

const (
	Master = "master"
	Worker = "worker"
)

type IP struct {
	identifier string
	value      string
}

type Node struct {
	role         string
	ip           IP
	kubeconfig   string
	user         string
	ami          string
	instanceType string
	keyName      string
}

func provisionBucket(ctx *pulumi.Context) error {
	bucket, err := s3.NewBucket(ctx, "infralink", &s3.BucketArgs{
		Acl: pulumi.String("private"),
		Tags: pulumi.StringMap{
			"Name": pulumi.String("infralink"),
		},
	})

	ctx.Export("bucket", bucket.Bucket)

	return err
}

func (n *Node) setupK0s(ctx context.Context) error {
	ansiblePlaybookConnectionOptions := &options.AnsibleConnectionOptions{
		User: n.user,
	}

	ansiblePlaybookOptions := &playbook.AnsiblePlaybookOptions{
		Inventory: fmt.Sprintf("%s,", n.ip.value), //That comma is required
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

func main() {
	ctx := context.Background()
	master := Node{
		role:         Master,
		user:         "ubuntu",
		instanceType: "t4g.micro",
		ami:          "ami-0b5801d081fa3a76c",
		keyName:      "martins.eglitis",
		ip: IP{
			identifier: fmt.Sprintf("%s-%s", Master, "ip"),
		},
	}
	worker := Node{
		role:         Worker,
		user:         "ubuntu",
		instanceType: "t4g.micro",
		ami:          "ami-0b5801d081fa3a76c",
		keyName:      "martins.eglitis",
		ip: IP{
			identifier: fmt.Sprintf("%s-%s", Worker, "ip"),
		},
	}

	stdoutStreamer := optup.ProgressStreams(os.Stdout)

	initialStack, err := auto.UpsertStackInlineSource(ctx, "aws", "infralink", func(ctx *pulumi.Context) error {
		return provisionBucket(ctx)
	})
	if err != nil {
		log.Fatal(err)
	}

	wsInitial := initialStack.Workspace()

	err = wsInitial.InstallPlugin(ctx, "aws", "v6.0.3")
	if err != nil {
		log.Fatal(err)
	}

	_, err = initialStack.Refresh(ctx)
	if err != nil {
		log.Fatal(err)
	}

	initialUpResult, err := initialStack.Up(ctx, stdoutStreamer)
	if err != nil {
		log.Fatal(err)
	}

	err = os.Setenv("PULUMI_BACKEND_URL", fmt.Sprintf("s3://%s", initialUpResult.Outputs["bucket"].Value))
	if err != nil {
		log.Fatal(err)
	}

	secondaryStack, err := auto.UpsertStackInlineSource(ctx, "aws", "infralink", func(ctx *pulumi.Context) error {
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

		//TODO - max open, no security
		securityGroup, err := ec2.NewSecurityGroup(ctx, "security-group", &ec2.SecurityGroupArgs{
			Ingress: ec2.SecurityGroupIngressArray{
				&ec2.SecurityGroupIngressArgs{
					FromPort: pulumi.Int(0),
					ToPort:   pulumi.Int(0),
					Protocol: pulumi.String("-1"),
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

		key, err := os.ReadFile("/home/martins/.ssh/id_rsa.pub")
		if err != nil {
			return err
		}

		keypair, err := ec2.NewKeyPair(ctx, "keypair", &ec2.KeyPairArgs{
			KeyName:   pulumi.String("replace-me"),
			PublicKey: pulumi.String(key),
		})
		if err != nil {
			return err
		}

		//Master setup
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

		masterInstance, err := ec2.NewInstance(ctx, "instance-master", &ec2.InstanceArgs{
			KeyName:      keypair.KeyName,
			Ami:          pulumi.String(master.ami),
			InstanceType: pulumi.String(master.instanceType),
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

		ctx.Export(master.ip.identifier, masterInstance.PublicIp)

		//Worker setup
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

		workerInstance, err := ec2.NewInstance(ctx, "instance-worker", &ec2.InstanceArgs{
			KeyName:      keypair.KeyName,
			Ami:          pulumi.String(worker.ami),
			InstanceType: pulumi.String(worker.instanceType),
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

		ctx.Export(worker.ip.identifier, workerInstance.PublicIp)

		//_, err = ec2.NewInstance(ctx, "master-test", &ec2.InstanceArgs{
		//	KeyName:      pulumi.String("martins.eglitis"),
		//	Ami:          pulumi.String(master.ami),
		//	InstanceType: pulumi.String(master.instanceType),
		//	Tags: pulumi.StringMap{
		//		"Name": pulumi.String("master-test"),
		//	},
		//})
		//if err != nil {
		//	return err
		//}
		//
		//_, err = ec2.NewInstance(ctx, "worker-test", &ec2.InstanceArgs{
		//	KeyName:      pulumi.String("martins.eglitis"),
		//	Ami:          pulumi.String(worker.ami),
		//	InstanceType: pulumi.String(worker.instanceType),
		//	Tags: pulumi.StringMap{
		//		"Name": pulumi.String("worker-test"),
		//	},
		//})
		//if err != nil {
		//	return err
		//}

		return nil
	})
	if err != nil {
		log.Fatal(err)
	}

	wsSecondary := secondaryStack.Workspace()

	err = wsSecondary.InstallPlugin(ctx, "aws", "v6.0.3")
	if err != nil {
		log.Fatal(err)
	}

	_, err = secondaryStack.Refresh(ctx)
	if err != nil {
		log.Fatal(err)
	}

	secondaryUpResult, err := secondaryStack.Up(ctx, stdoutStreamer)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(secondaryUpResult.Outputs)

	master.ip.value = fmt.Sprintf("%s", secondaryUpResult.Outputs[master.ip.identifier].Value)

	err = master.setupK0s(ctx)
	if err != nil {
		log.Fatal(err)
	}

	worker.ip.value = fmt.Sprintf("%s", secondaryUpResult.Outputs[worker.ip.identifier].Value)

	err = worker.setupK0s(ctx)
	if err != nil {
		log.Fatal(err)
	}
}
