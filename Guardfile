# notification :gntp, host: '192.168.1.7'
notification :gntp, host: '172.20.10.3'

guard :shell do
  watch(/.*\/(.*\/)?(.*)\.rs$/) do
    `cargo test --features inline-systemd`
    if $? == 0 
      n "specinfra - cargo test success", "cargo test", :success
    else
      n "specinfra - cargo test failed", "cargo test", :failed
    end
  end
end
