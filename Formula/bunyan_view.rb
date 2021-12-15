class BunyanView < Formula
  desc "Cross-platform Text Expander written in Rust"
  homepage "https://github.com/dekobon/bunyan-view"
  url "https://github.com/federico-terzi/espanso/releases/latest/download/espanso-mac.tar.gz"
  sha256 "2473866b99eef9ea983200b7aac91592b938404ffaa1fb8c72beacb2ebd3203a"
  version "0.2.1"

  def install
    bin.install "bunyan"
    man1.install "bunyan.1.gz"
  end
end