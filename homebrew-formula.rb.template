class Muv < Formula
  desc "Global environment management tool using uv"
  homepage "https://github.com/vineel7871/muv"
  version "{{VERSION}}"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/vineel7871/muv/releases/download/v{{VERSION}}/muv-macos-arm64.tar.gz"
      sha256 "{{SHA256_MACOS_ARM64}}"
    else
      url "https://github.com/vineel7871/muv/releases/download/v{{VERSION}}/muv-macos-amd64.tar.gz"
      sha256 "{{SHA256_MACOS_AMD64}}"
    end
  end

  def install
    bin.install "muv"
  end

  test do
    system "#{bin}/muv", "--version"
  end
end