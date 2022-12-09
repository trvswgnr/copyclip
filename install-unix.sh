# Path: install-unix.sh
#!/bin/sh

install() {
  filename="copyclip"
  api_url="https://api.github.com/repos/trvswgnr/copyclip/releases/latest"
  echo "Getting download url from $api_url..."
  # get the browser_download_url from the latest release for files with the name copyclip
  download_url=$(curl -s $api_url | grep "browser_download_url.*$filename" | cut -d '"' -f 4)
  echo "Getting latest release from $download_url..."
  
  # download the latest release to the system's temp directory
  echo "Creating temp directory..."
  temp_dir=$(mktemp -d)
  echo "Downloading $filename to $temp_dir/ ..."
  curl -L $download_url -o $temp_dir/$filename
  # make the file executable
  echo "Making $filename executable..."
  chmod +x $temp_dir/$filename
  
  # move the file to folder where the user can execute it
  bin_dir="/usr/local/bin"
  echo "Moving $filename to $bin_dir/ ..."
  mv $temp_dir/$filename $bin_dir/$filename
}

# check if install has any errors
if install; then
  echo "Installation successful!"
  echo "\nUsage: echo 'Hello World' | copyclip"
else
  echo "Failed to install"
fi

# log the output of cargo build --target aarch64-pc-windows-msvc to a file
echo "Building for Windows on ARM..."
cargo build --target aarch64-pc-windows-msvc > build.log 2>&1
