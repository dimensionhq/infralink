output "master" {
  value = module.master.public_ip
}

output "worker" {
  value = module.worker.public_ip
}