guard :shell do
  watch(/.*\/(.*\/)?(.*)\.rs$/) do
    `cargo test`
    if $? == 0 
      n "cargo test success", "cargo test", :success
    else
      n "cargo test failed", "cargo test", :failed
    end
  end
end
