package main

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ecr"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

type Common struct {
	name         string
	key          string
	user         string
	ami          string
	instanceType string
	verbose      bool
	configDir    string
	region       string
}

type Node struct {
	role       string
	ip         string
	kubeconfig string
	hostname   string
}

func upsertLocalStack(ctx *pulumi.Context, common Common) error {
	bucket, err := s3.NewBucket(ctx, common.name, &s3.BucketArgs{
		Acl: pulumi.String("private"),
	})
	if err != nil {
		return err
	}

	ctx.Export("bucket", bucket.Bucket)

	return nil
}

func upsertRemoteStack(ctx *pulumi.Context, common Common, master Node, worker Node) error {
	region, err := aws.GetRegion(ctx, nil, nil)
	if err != nil {
		return err
	}

	ctx.Export("region", pulumi.String(region.Name))

	zones, err := aws.GetAvailabilityZones(ctx, nil)
	if err != nil {
		return err
	}

	vpc, err := ec2.NewVpc(ctx, "vpc", &ec2.VpcArgs{
		CidrBlock: pulumi.StringPtr("172.0.0.0/16"),
		Tags: pulumi.StringMap{
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
		},
	})
	if err != nil {
		return err
	}

	gateway, err := ec2.NewInternetGateway(ctx, "gateway", &ec2.InternetGatewayArgs{
		VpcId: vpc.ID(),
		Tags: pulumi.StringMap{
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
		},
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
			&ec2.SecurityGroupIngressArgs{
				FromPort: pulumi.Int(6443),
				ToPort:   pulumi.Int(6443),
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
		Tags: pulumi.StringMap{
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
		},
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
		Tags: pulumi.StringMap{
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
		},
	})
	if err != nil {
		return err
	}

	repository, err := ecr.NewRepository(ctx, "repository", &ecr.RepositoryArgs{
		Name: pulumi.String(common.name),
		Tags: pulumi.StringMap{
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
		},
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
		Tags: pulumi.StringMap{
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
		},
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
		Tags: pulumi.StringMap{
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
		},
	})

	_, err = ec2.NewRouteTableAssociation(ctx, "route-table-association-master", &ec2.RouteTableAssociationArgs{
		RouteTableId: routeTableMaster.ID(),
		SubnetId:     subnetMaster.ID(),
	})

	tmpJSON, err := json.Marshal(map[string]interface{}{
		"Version": "2012-10-17",
		"Statement": []map[string]interface{}{
			{
				"Action": "sts:AssumeRole",
				"Effect": "Allow",
				"Sid":    "",
				"Principal": map[string]interface{}{
					"Service": "ec2.amazonaws.com",
				},
			},
		},
	})
	if err != nil {
		return err
	}
	_, err = iam.NewRole(ctx, "role-master", &iam.RoleArgs{
		AssumeRolePolicy: pulumi.String(tmpJSON),
		Tags: pulumi.StringMap{
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
		},
	})
	if err != nil {
		return err
	}

	masterInstance, err := ec2.NewInstance(ctx, "instance-master", &ec2.InstanceArgs{
		KeyName:      keypair.KeyName,
		Ami:          pulumi.String(common.ami),
		InstanceType: pulumi.String(common.instanceType),
		Tags: pulumi.StringMap{
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
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

	ctx.Export("master-ip-public", masterInstance.PublicIp)
	ctx.Export("master-ip-private", masterInstance.PrivateIp)

	//Worker setup
	//TODO - move to a common func
	subnetWorker, err := ec2.NewSubnet(ctx, "subnet-worker", &ec2.SubnetArgs{
		AvailabilityZone:    pulumi.StringPtr(zones.Names[1]),
		VpcId:               vpc.ID(),
		CidrBlock:           pulumi.StringPtr("172.0.2.0/24"),
		MapPublicIpOnLaunch: pulumi.Bool(true),
		Tags: pulumi.StringMap{
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
		},
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
		Tags: pulumi.StringMap{
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
		},
	})

	_, err = ec2.NewRouteTableAssociation(ctx, "route-table-association-worker", &ec2.RouteTableAssociationArgs{
		RouteTableId: routeTableWorker.ID(),
		SubnetId:     subnetWorker.ID(),
	})

	workerInstance, err := ec2.NewInstance(ctx, "instance-worker", &ec2.InstanceArgs{
		KeyName:      keypair.KeyName,
		Ami:          pulumi.String(common.ami),
		InstanceType: pulumi.String(common.instanceType),
		Tags: pulumi.StringMap{
			"Name": pulumi.String(worker.role),
			fmt.Sprintf("kubernetes.io/cluster/%s", common.name): pulumi.String("owned"),
		},
		SubnetId: subnetWorker.ID(),
		VpcSecurityGroupIds: pulumi.StringArray{
			securityGroup.ID(),
		},
	})
	if err != nil {
		return err
	}

	ctx.Export("worker-ip-public", workerInstance.PublicIp)
	ctx.Export("worker-ip-private", workerInstance.PrivateIp)

	return nil
}
