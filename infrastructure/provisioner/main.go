package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"log"
	"os"
)

func setupMaster() error {
	contents, err := os.ReadFile("master.sh")
	if err != nil {
		log.Fatal(err)
	}

	//var ip pulumi.StringOutput

	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ec2.NewInstance(ctx, "web", &ec2.InstanceArgs{
			KeyName:      pulumi.String("martins.eglitis"),
			Ami:          pulumi.String("ami-0b5801d081fa3a76c"),
			InstanceType: pulumi.String("t4g.micro"),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("master"),
			},
			UserData: pulumi.String(contents),
		})

		//ip = instance.PublicIp

		return err
	})

	return nil
}

func setupWorker() {
	contents, err := os.ReadFile("worker.sh")
	if err != nil {
		log.Fatal(err)
	}

	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ec2.NewInstance(ctx, "web", &ec2.InstanceArgs{
			KeyName:      pulumi.String("martins.eglitis"),
			Ami:          pulumi.String("ami-0b5801d081fa3a76c"),
			InstanceType: pulumi.String("t4g.micro"),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("worker"),
			},
			UserData: pulumi.String(contents),
		})
		if err != nil {
			return err
		}
		return nil
	})
}

func preSetup() {
	//TODO - see an example here: https://github.com/k0sproject/k0s/blob/main/docs/custom-ca.md
	//TODO - have to use libraries for generating new x509 certificates
}

func main() {
	preSetup()
	setupMaster()
	//setupWorker()
}
