class BunyanView < Formula
  desc "A full-featured port of the Node Bunyan structured log file viewer"
  homepage "https://github.com/dekobon/bunyan-view"
  version "0.3.0"

  if OS.mac? and Hardware::CPU.intel?
      url "https://github.com/dekobon/bunyan-view/releases/download/v#{version}/bunyan-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "4df39716ae6024ab68d877343afcdd3ef6f70a175a4f2638900942ce211dfd13"
  elsif OS.linux? and Hardware::CPU.intel?
      url "https://github.com/dekobon/bunyan-view/releases/download/v#{version}/bunyan-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "aed5dc69a9fc5e9208c1fdb14237ae329dbfbaa6cb1c972767fe4da142cd5558"
  end


  def install
    bin.install "bunyan"
#    man1.install "bunyan.1.gz"
  end
end
