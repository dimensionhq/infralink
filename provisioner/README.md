# provisioner

A CLI tool for provisioning K0s clusters.

## Quick start

1. Make sure you have `kubectl` and `go` installed.
2. Login to your AWS account and switch to the profile you want to provision your cluster into.
3. Run `go run main.go`.
4. Check if everything is working `kubectl --kubeconfig assets/playbooks/tmp/kubeconfig get nodes`
5. Everything is up-and-ready if you see similar output to this:
```
NAME               STATUS   ROLES    AGE     VERSION
ip-172-31-43-154   Ready    <none>   3m18s   v1.27.5+k0s
```