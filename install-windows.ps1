# Path: install-windows.ps1

# function to install the latest release of the "copyclip" utility
function Install {
  # set the name of the file to download
  $filename = "copyclip"

  # set the URL of the latest release on GitHub
  $api_url = "https://api.github.com/repos/trvswgnr/copyclip/releases/latest"

  # show a message indicating that the download URL is being retrieved
  Write-Output "Getting download url from $api_url..."

  # use PowerShell's Invoke-WebRequest cmdlet to download the JSON data from the specified URL
  $json = Invoke-WebRequest -Uri $api_url

  # parse the JSON data to get the download URL for the file with the name specified in the $filename variable
  $download_url = ($json | ConvertFrom-Json).assets | Where-Object { $_.name -eq $filename } | Select-Object -ExpandProperty browser_download_url

  # show a message indicating that the download is starting
  Write-Output "Getting latest release from $download_url..."

  # create a temporary directory to download the file to
  $temp_dir = New-TemporaryFile
  $temp_dir = Split-Path $temp_dir

  # download the file to the temporary directory
  Invoke-WebRequest -Uri $download_url -OutFile "$temp_dir/$filename"

  # move the file to the user's local bin directory
  $bin_dir = "$env:LOCALAPPDATA\bin"
  if (!(Test-Path $bin_dir)) {
    # create the bin directory if it doesn't exist
    New-Item -ItemType Directory -Path $bin_dir
  }

  # add the bin directory to the user's PATH environment variable
  $env:PATH += ";$bin_dir"

  # move the file to the bin directory
  Move-Item "$temp_dir/$filename" "$bin_dir/$filename"
}

# check if the install function ran successfully
if (Install) {
  # show a success message
  Write-Output "Installation successful!"
  Write-Output "\nUsage: echo 'Hello World' | copyclip"
} else {
  # show an error message
  Write-Output "Failed to install"
}