package main

import (
	"context"
	"fmt"
	"github.com/apenella/go-ansible/pkg/options"
	"github.com/apenella/go-ansible/pkg/playbook"
)

func (n *Node) setupK0s(ctx context.Context, common Common) error {
	ansiblePlaybookConnectionOptions := &options.AnsibleConnectionOptions{
		User: common.user,
	}

	ansiblePlaybookOptions := &playbook.AnsiblePlaybookOptions{
		Inventory: fmt.Sprintf("%s,", n.ip), //That comma is required
		ExtraVars: map[string]interface{}{
			"config_dir": common.configDir,
			"hostname":   n.hostname,
		},
	}

	privilegeEscalationOptions := &options.AnsiblePrivilegeEscalationOptions{
		Become: true,
	}

	playbookCmd := &playbook.AnsiblePlaybookCmd{
		Playbooks: []string{
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
