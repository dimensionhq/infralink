package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"github.com/apenella/go-ansible/pkg/options"
	"github.com/apenella/go-ansible/pkg/playbook"
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/ec2"
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

func (n *Node) provisionInstance(ctx *pulumi.Context) error {
	instance, err := ec2.NewInstance(ctx, n.role, &ec2.InstanceArgs{
		KeyName:      pulumi.String(n.keyName),
		Ami:          pulumi.String(n.ami),
		InstanceType: pulumi.String(n.instanceType),
		Tags: pulumi.StringMap{
			"Name": pulumi.String(n.role),
		},
	})

	ctx.Export(n.ip.identifier, instance.PublicIp)

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
		ip: IP{
			identifier: fmt.Sprintf("%s-%s", Master, "ip"),
		},
	}
	worker := Node{
		role:         Worker,
		user:         "ubuntu",
		instanceType: "t4g.micro",
		ami:          "ami-0b5801d081fa3a76c",
		ip: IP{
			identifier: fmt.Sprintf("%s-%s", Worker, "ip"),
		},
	}

	deployFunc := func(ctx *pulumi.Context) error {
		var err error

		err = master.provisionInstance(ctx)
		if err != nil {
			return err
		}

		err = worker.provisionInstance(ctx)
		if err != nil {
			return err
		}

		return nil
	}

	stack, err := auto.UpsertStackInlineSource(ctx, "aws", "infralink", deployFunc)
	if err != nil {
		log.Fatal(err)
	}

	workspace := stack.Workspace()

	err = workspace.InstallPlugin(ctx, "aws", "v6.0.3")
	if err != nil {
		log.Fatal(err)
	}

	_, err = stack.Refresh(ctx)
	if err != nil {
		log.Fatal(err)
	}

	stdoutStreamer := optup.ProgressStreams(os.Stdout)

	upRes, err := stack.Up(ctx, stdoutStreamer)
	if err != nil {
		log.Fatal(err)
	}

	master.ip.value = fmt.Sprintf("%s", upRes.Outputs[master.ip.identifier].Value)

	err = master.setupK0s(ctx)
	if err != nil {
		log.Fatal(err)
	}

	worker.ip.value = fmt.Sprintf("%s", upRes.Outputs[worker.ip.identifier].Value)

	err = worker.setupK0s(ctx)
	if err != nil {
		log.Fatal(err)
	}
}
