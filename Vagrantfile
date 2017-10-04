# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.define :xenial64 do |c|
    c.vm.box = "ubuntu/xenial64"
    c.vm.provision :shell, inline: <<-EOF
      curl https://sh.rustup.rs -sSf | su -l ubuntu -c "sh -s -- -y"
      apt-get update
      apt-get install -y gcc libssl-dev pkg-config cmake
    EOF
  end

  config.vm.define :centos7 do |c|
    c.vm.box = "centos/7"
    c.vm.provision :shell, inline: <<-EOF
      curl https://sh.rustup.rs -sSf | su -l vagrant -c "sh -s -- -y"
      yum install -y gcc openssl-devel cmake
    EOF
  end

  config.vm.synced_folder ".", "/vagrant", type: "virtualbox"
end
