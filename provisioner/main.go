package main

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/fsnotify/fsnotify"
	"github.com/pulumi/pulumi/sdk/v3/go/auto"
	"github.com/pulumi/pulumi/sdk/v3/go/auto/optup"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/spf13/pflag"
	"github.com/spf13/viper"
)

type auth struct {
	Auth string `json:"auth"`
}

type dockerConfig struct {
	Auths map[string]auth `json:"auths"`
}

func check(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

func main() {
	homeDir, err := os.UserHomeDir()
	check(err)

	configDir := filepath.Join(homeDir, ".infralink")
	configFile := filepath.Join(configDir, "config.toml")
	dockerFile := filepath.Join(configDir, "config.json")
	kubeconfigFile := filepath.Join(configDir, "kubeconfig")

	pflag.String("name", "infralink", "AWS resource names/tags")
	pflag.String("instance", "t3a.micro", "AWS EC2 instance type") //TODO - find a way to make AMI with packer for ARM
	pflag.String("ami", "ami-09ca9cb836d95b14c", "AWS AMI ID")
	pflag.String("user", "ubuntu", "AWS EC2 system user")
	pflag.String("key", filepath.Join(homeDir, ".ssh", "id_rsa.pub"), "SSH public key path")
	pflag.Bool("verbose", false, "toggle verbosity (warning: outputs sensitive information)")

	//CLI
	pflag.Parse()
	err = viper.BindPFlags(pflag.CommandLine)
	check(err)

	//Environment
	viper.SetEnvPrefix("INFRALINK")
	viper.AutomaticEnv()
	replacer := strings.NewReplacer("-", "_")
	viper.SetEnvKeyReplacer(replacer)

	//File
	_, err = os.ReadFile(configFile)
	if err == nil {
		viper.SetConfigFile(configFile)
		err = viper.ReadInConfig()
		check(err)

		viper.OnConfigChange(func(e fsnotify.Event) {
			fmt.Println("Config file changed:", e.Name)
		})

		viper.WatchConfig()
	} else if errors.Is(err, os.ErrNotExist) {
		err = os.Mkdir(configDir, 0700)
		if !errors.Is(err, os.ErrExist) {
			check(err)
		}

		file, err := os.OpenFile(configFile, os.O_CREATE, 0600)
		check(err)
		defer file.Close()
	} else {
		check(err)
	}

	name := viper.GetString("name")
	ami := viper.GetString("ami")
	instance := viper.GetString("instance")
	user := viper.GetString("user")
	key := viper.GetString("key")
	verbose := viper.GetBool("verbose")

	common := Common{
		name:     name,
		key:      key,
		instance: instance,
		user:     user,
		ami:      ami,
		verbose:  verbose,
	}
	master := Node{
		role: "master",
	}
	worker := Node{
		role: "worker",
	}

	ctx := context.Background()

	//TODO - find a better way
	err = os.Setenv("PULUMI_BACKEND_URL", fmt.Sprintf("file://%s", configDir))
	check(err)

	//TODO - find a better way, do not hardcode?
	err = os.Setenv("PULUMI_CONFIG_PASSPHRASE", "your-passphrase")
	check(err)

	initialStack, err := auto.UpsertStackInlineSource(ctx, "aws", "infralink", func(ctx *pulumi.Context) error {
		return upsertLocalStack(ctx, common)
	})
	check(err)

	wsInitial := initialStack.Workspace()

	err = wsInitial.InstallPlugin(ctx, "aws", "v6.0.3")
	check(err)

	_, err = initialStack.Refresh(ctx)
	check(err)

	var initialUpResult auto.UpResult

	if verbose {
		initialUpResult, err = initialStack.Up(ctx, optup.ProgressStreams(os.Stdout))
	} else {
		initialUpResult, err = initialStack.Up(ctx)
	}
	check(err)

	//TODO - find a better way
	err = os.Setenv("PULUMI_BACKEND_URL", fmt.Sprintf("s3://%s", initialUpResult.Outputs["bucket"].Value))
	check(err)

	secondaryStack, err := auto.UpsertStackInlineSource(ctx, "aws", "infralink", func(ctx *pulumi.Context) error {
		return upsertRemoteStack(ctx, common, master, worker)
	})
	check(err)

	wsSecondary := secondaryStack.Workspace()

	err = wsSecondary.InstallPlugin(ctx, "aws", "v6.0.3")
	check(err)

	_, err = secondaryStack.Refresh(ctx)
	check(err)

	var secondaryUpResult auto.UpResult

	if verbose {
		secondaryUpResult, err = secondaryStack.Up(ctx, optup.ProgressStreams(os.Stdout))
	} else {
		secondaryUpResult, err = secondaryStack.Up(ctx)
	}
	check(err)

	//TODO - probably a bad way of checking if smth is working/not working
	kf, err := os.OpenFile(kubeconfigFile, os.O_RDONLY, 0600)
	if err != nil {
		master.ip = fmt.Sprintf("%s", secondaryUpResult.Outputs["master-ip"].Value)

		err = master.setupK0s(ctx, common, configDir)
		check(err)

		worker.ip = fmt.Sprintf("%s", secondaryUpResult.Outputs["worker-ip"].Value)

		err = worker.setupK0s(ctx, common, configDir)
		check(err)
	}
	defer kf.Close()

	repository := fmt.Sprintf("%s", secondaryUpResult.Outputs["repository"].Value)
	username := fmt.Sprintf("%s", secondaryUpResult.Outputs["username"].Value)
	password := fmt.Sprintf("%s", secondaryUpResult.Outputs["password"].Value)

	dockerConfigs := dockerConfig{Auths: map[string]auth{
		repository: {
			Auth: base64.StdEncoding.EncodeToString([]byte(fmt.Sprintf("%s:%s", username, password))),
		},
	}}

	configJSON, err := json.MarshalIndent(dockerConfigs, "", "    ")
	if err != nil {
		fmt.Printf("Error marshaling JSON: %s\n", err)
		os.Exit(1)
	}

	df, err := os.OpenFile(dockerFile, os.O_CREATE|os.O_TRUNC|os.O_WRONLY, 0600)
	check(err)
	defer df.Close()

	_, err = df.WriteString(fmt.Sprintf("%s", configJSON))
	check(err)
}
