class Speedo < Formula
  desc "Network speed test tool and file downloader built in Rust"
  homepage "https://github.com/coryzibell/speedo"
  version "0.2.11"
  license "MIT OR Apache-2.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/coryzibell/speedo/releases/download/v0.2.11/speedo-macos-aarch64.tar.gz"
      sha256 "REPLACE_WITH_ARM64_SHA256"
    else
      url "https://github.com/coryzibell/speedo/releases/download/v0.2.11/speedo-macos-x86_64.tar.gz"
      sha256 "REPLACE_WITH_X86_64_SHA256"
    end
  end

  def install
    bin.install "speedo"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/speedo --version")
  end
end
