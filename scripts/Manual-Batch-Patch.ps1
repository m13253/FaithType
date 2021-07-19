#!/usr/bin/env pwsh

<# .SYNOPSIS #>

param (
    [Parameter(Mandatory = $true, Position = 0, ValueFromRemainingArguments = $true)]
    [String[]]
    # Path to input files, supports wildcards.
    $InputFiles,

    [String]
    [Parameter(Mandatory = $true)]
    # Path to output files.
    $OutputDir = 'C:\Patched Fonts'
)

Set-StrictMode -Version 3.0

$FaithTypeCmd = Join-Path -Path $PSScriptRoot -ChildPath 'faithtype.exe' -ErrorAction Stop
$FaithTypeArgs = @()
if (-not (Test-Path -Path $FaithTypeCmd -PathType Leaf)) {
    $FaithTypeCmd = if ($null -ne $Env:CARGO -and $Env:CARGO -ne '') {
        $Env:CARGO
    } else {
        'cargo.exe'
    }
    $CargoTomlPath = Join-Path -Path $PSScriptRoot -ChildPath '..\Cargo.toml'
    $FaithTypeArgs = 'run', '--bin', 'faithtype', '--manifest-path', """$($CargoTomlPath.replace('"', '\"'))""", '--quiet', '--release', '--'
}

New-Item -Path $OutputDir -ItemType Directory -ErrorAction Ignore | Out-Null

foreach ($InputPattern in $InputFiles) {
    foreach ($InputFile in Get-Item $InputPattern | Where-Object {
            -not $_.PSIsContainer
        } | Sort-Object ) {
        if (-not (Test-Path -Path $InputFile -PathType Leaf)) {
            return
        }
        $InputFileName = $InputFile.FullName
        $OutputFileName = Join-Path -Path $OutputDir -ChildPath $InputFile.Name -ErrorAction Stop
        Write-Host "> faithtype -o ""$($OutputFileName.replace('"', '\"'))"" -- ""$($InputFileName.replace('"', '\"'))"""
        Start-Process -FilePath $FaithTypeCmd -ArgumentList ($FaithTypeArgs + ('-o', """$($OutputFileName.replace('"', '\"'))""", '--', """$($InputFileName.replace('"', '\"'))""")) -NoNewWindow -Wait -ErrorAction Stop
        Write-Host
    }
}
