kubectl expose deployment ingress-nginx-controller --target-port=80 --type=ClusterIP -n kube-system
