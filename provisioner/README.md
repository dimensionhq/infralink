# provisioner

A CLI tool for provisioning a K0s cluster on AWS.

## Quick start

1. Make sure you have `awsctl`, `go`, and `kubectl` installed.
2. Login to your AWS account, acquire your IAM key and secret. Then switch to the profile you want to provision your cluster into. You can use `aws configure` to set up your profile.
3. Build the binary with `go build .`.
4. Run `./provisioner`.
5. Check if everything is working `kubectl --kubeconfig ~/.infralink/kubeconfig get nodes`
6. Everything is up-and-ready if you see similar output to this:
```
NAME               STATUS   ROLES    AGE     VERSION
ip-172-31-43-154   Ready    <none>   3m18s   v1.27.5+k0s
```

## Configuration

You can edit the configuration file under `~/.infralink/config.toml`, e.g.

```toml
ami='ami-09ca9cb836d95b14c'
instance='t3a.micro'
key='~/.ssh/id_rsa.pub'
name='infralink'
user='ubuntu'
verbose=true
```

or you can pass the respective environment variables:

```dotenv
INFRALINK_AMI='ami-09ca9cb836d95b14c'
INFRALINK_INSTANCE='t3a.micro'
INFRALINK_KEY='~/.ssh/id_rsa.pub'
INFRALINK_NAME='infralink'
INFRALINK_USER='ubuntu'
INFRALINK_VERBOSE=true
```