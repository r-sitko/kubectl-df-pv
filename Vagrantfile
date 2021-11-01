Vagrant.configure("2") do |config|
    config.vm.box = "debian/bullseye64"
    config.vm.box_version = "11.20210829.1"
    config.vm.provider "virtualbox" do |vb|
        vb.memory = "2500"
        vb.cpus = "4"
    end

    config.vm.provision "shell", inline: "swapoff -a"

    config.vm.provision "apt-get update", type: "shell", inline: <<-SCRIPT
echo "apt-get update"
apt-get update
SCRIPT

    config.vm.provision "rust", type: "shell", inline: <<-SCRIPT
echo "Installing Rust"
apt-get install -y build-essential pkg-config libssl-dev curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sudo -u vagrant sh -s -- -y
sudo -u vagrant export CARGO_TARGET_DIR=../home/vagrant/cargo
SCRIPT

    config.vm.provision "lvm2", type: "shell", inline: <<-SCRIPT
echo "Installing LVM2"
apt-get install -y lvm2
SCRIPT

    config.vm.provision :docker

    config.vm.provision "minikube", type: "shell", inline: <<-SCRIPT
echo "Installing Minikube"
curl -LO https://github.com/kubernetes/minikube/releases/download/v1.23.2/minikube-linux-amd64
install minikube-linux-amd64 /usr/local/bin/minikube
SCRIPT

    config.vm.provision "conntrack", type: "shell", inline: <<-SCRIPT
echo "Installing Conntrack"
apt-get install -y conntrack
SCRIPT

    config.vm.provision "kubectl", type: "shell", inline: <<-SCRIPT
echo "Installing Kubectl"
curl -LO https://dl.k8s.io/release/v1.22.2/bin/linux/amd64/kubectl
install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
SCRIPT

    config.vm.provision "helm", type: "shell", inline: <<-SCRIPT
echo "Installing Helm"
curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3
chmod 700 get_helm.sh
./get_helm.sh
SCRIPT

    config.vm.provision "loop0", type: "shell", inline: <<-SCRIPT
echo "Setup /dev/loop0"
truncate -s 2G /disk.img
losetup -f /disk.img --show
pvcreate /dev/loop0
vgcreate lvmvg /dev/loop0
SCRIPT

    config.vm.provision "kubernetes", type: "shell", privileged: false, inline: <<-SCRIPT
echo "Starting Kubernetes cluster"
minikube start --vm-driver docker --kubernetes-version v1.22.2 --nodes 2
kubectl cluster-info
minikube addons enable pod-security-policy
SCRIPT

    config.vm.provision "openebs", type: "shell", privileged: false, inline: <<-SCRIPT
echo "Deploying OpenEBS"
helm repo add openebs-lvmlocalpv https://openebs.github.io/lvm-localpv
helm repo update
helm install openebs-lvmlocalpv openebs-lvmlocalpv/lvm-localpv --version 0.8.5 --namespace openebs --create-namespace --wait

cat <<EOF | kubectl create -f -
apiVersion: storage.k8s.io/v1
kind: StorageClass
metadata:
  name: openebs-lvmpv
parameters:
  storage: "lvm"
  volgroup: "lvmvg"
provisioner: local.csi.openebs.io
volumeBindingMode: WaitForFirstConsumer
EOF
SCRIPT

    config.vm.provision "apt-get clean", type: "shell", inline: <<-SCRIPT
echo "apt-get clean"
apt-get clean
SCRIPT

end
