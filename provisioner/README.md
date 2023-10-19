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