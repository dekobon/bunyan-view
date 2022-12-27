class BunyanView < Formula
  desc "A full-featured Rust port of the Node Bunyan structured log file viewer"
  homepage "https://github.com/dekobon/bunyan-view"
  version "0.3.2"
  package_name = "bunyan-view"
  src_repo = "https://github.com/dekobon/bunyan-view"

  if OS.mac? and Hardware::CPU.intel?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_v#{version}_x86_64-apple-darwin.tar.gz"
      sha256 ""
  elsif OS.mac? and Hardware::CPU.arm?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_#{version}_aarch64-apple-darwin.tar.gz"
      sha256 ""
  elsif OS.linux? and Hardware::CPU.intel?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_#{version}_x86_64-unknown-linux-gnu.tar.gz"
      sha256 "d888006130b18fd9fa84d23ea71bd373c924e823d6ac6f74ffdcb93a4592bd4d"
  elsif OS.linux? and Hardware::CPU.arm? and Hardware::CPU.is_64_bit?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_#{version}_aarch64-unknown-linux-gnu.tar.gz"
      sha256 "8fa9a34497b2f1d3f042b8cf05110ff7b6b75129451d3edcc27d1dd8459356c7"
  else
      odie "Unsupported architecture"
  end


  def install
    bin.install "bunyan"
    man1.install "bunyan.1.gz"
  end
end
