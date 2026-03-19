class ReadmeSummarizer < Formula
  desc "CLI tool to scan directories and generate summaries of README files"
  homepage "https://github.com/YOUR_USERNAME/readme-summarizer"
  url "https://github.com/YOUR_USERNAME/readme-summarizer/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/readme-sum", "--version"
  end
end
