# This script takes care of packaging the build artifacts that will go in the
# release zipfile

$SRC_DIR = $PWD.Path
$STAGE = [System.Guid]::NewGuid().ToString()

Set-Location $args[0]
# build the specific item
cargo rustc --target %TARGET% --features "%FEATURES" --release --lib

Set-Location $ENV:Temp
New-Item -Type Directory -Name $STAGE
Set-Location $STAGE

$ZIP = "$SRC_DIR\$($args[0])$($Env:RELEASE_SUFFIX)-$($Env:APPVEYOR_REPO_TAG_NAME)-$($Env:TARGET_NAME).zip"

# TODO Update this to package the right artifacts
Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\$($args[0]).dll" '.\'
Copy-Item "$SRC_DIR\README.md" '.\'
Copy-Item "$SRC_DIR\LICENSE" '.\'

7z a "$ZIP" *

Push-AppveyorArtifact "$ZIP"

Remove-Item *.* -Force
Set-Location ..
Remove-Item $STAGE
Set-Location $SRC_DIR
