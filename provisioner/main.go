package main

import (
	"context"
	"flag"
	"fmt"
	"github.com/pulumi/pulumi/sdk/v3/go/auto/optup"
	"github.com/spf13/pflag"
	"log"
	"os"
	"strings"

	"github.com/fsnotify/fsnotify"
	"github.com/pulumi/pulumi/sdk/v3/go/auto"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/spf13/viper"
)

func getString(key string) string {
	if !viper.IsSet(key) {
		log.Fatalf("%s not set", key)
	}

	return viper.GetString(key)
}

func getBool(key string) bool {
	if !viper.IsSet(key) {
		log.Fatalf("%s not set", key)
	}

	return viper.GetBool(key)
}

func main() {
	pflag.String("name", "infralink", "name for some AWS resources")
	pflag.String("instance", "t4g.micro", "AWS EC2 instance type")
	pflag.String("ami", "ami-0b5801d081fa3a76c", "AWS AMI ID")
	pflag.String("user", "ubuntu", "AWS EC2 system user")
	pflag.String("key", "/path/to/your/id_rsa.pub", "SSH public key path")
	pflag.Bool("verbose", false, "toggle verbosity (warning: outputs sensitive information)")

	//TODO - find a way to get rid of the glog CLI flags
	//CLI
	pflag.CommandLine.AddGoFlagSet(flag.CommandLine)
	pflag.Parse()
	err := viper.BindPFlags(pflag.CommandLine)
	if err != nil {
		log.Fatal(err)
	}

	//Environment
	viper.SetEnvPrefix("INFRALINK")
	viper.AutomaticEnv()
	replacer := strings.NewReplacer("-", "_")
	viper.SetEnvKeyReplacer(replacer)

	//File
	viper.SetConfigFile("config.toml")
	err = viper.ReadInConfig()
	if err != nil {
		log.Fatal(err)
	}
	viper.OnConfigChange(func(e fsnotify.Event) {
		fmt.Println("Config file changed:", e.Name)
	})
	viper.WatchConfig()

	name := getString("name")
	ami := getString("ami")
	instance := getString("instance")
	user := getString("user")
	key := getString("key")
	verbose := getBool("verbose")

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
		role: "master",
	}

	ctx := context.Background()

	initialStack, err := auto.UpsertStackInlineSource(ctx, "aws", "infralink", func(ctx *pulumi.Context) error {
		return upsertInitialStack(ctx, common)
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

	var initialUpResult auto.UpResult

	if verbose {
		initialUpResult, err = initialStack.Up(ctx, optup.ProgressStreams(os.Stdout))
	} else {
		initialUpResult, err = initialStack.Up(ctx)
	}

	if err != nil {
		log.Fatal(err)
	}

	err = os.Setenv("PULUMI_BACKEND_URL", fmt.Sprintf("s3://%s", initialUpResult.Outputs["bucket"].Value))
	if err != nil {
		log.Fatal(err)
	}

	secondaryStack, err := auto.UpsertStackInlineSource(ctx, "aws", "infralink", func(ctx *pulumi.Context) error {
		return upsertSecondaryStack(ctx, common, master, worker)
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

	var secondaryUpResult auto.UpResult

	if verbose {
		secondaryUpResult, err = secondaryStack.Up(ctx, optup.ProgressStreams(os.Stdout))
	} else {
		secondaryUpResult, err = secondaryStack.Up(ctx)
	}
	if err != nil {
		log.Fatal(err)
	}

	master.ip = fmt.Sprintf("%s", secondaryUpResult.Outputs["master-ip"].Value)

	err = master.setupK0s(ctx, common)
	if err != nil {
		log.Fatal(err)
	}

	worker.ip = fmt.Sprintf("%s", secondaryUpResult.Outputs["worker-ip"].Value)

	err = worker.setupK0s(ctx, common)
	if err != nil {
		log.Fatal(err)
	}

	repository := fmt.Sprintf("%s", secondaryUpResult.Outputs["repository"].Value)
	username := fmt.Sprintf("%s", secondaryUpResult.Outputs["username"].Value)
	password := fmt.Sprintf("%s", secondaryUpResult.Outputs["password"].Value)
	token := fmt.Sprintf("%s", secondaryUpResult.Outputs["token"].Value)

	err = pushImage(repository, username, password, token)
	if err != nil {
		log.Fatal(err)
	}
}
