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

$FaithTypeCmd = '.\faithtype.exe'
$FaithTypeArgs = @()
if (-not (Test-Path -Path $FaithTypeCmd -PathType Leaf)) {
    $FaithTypeCmd = if ($null -ne $Env:CARGO -and $Env:CARGO -ne '') {
        $Env:CARGO
    } else {
        'cargo.exe'
    }
    $FaithTypeArgs = 'run', '--bin', 'faithtype', '--manifest-path', '..\Cargo.toml', '--quiet', '--release', '--'
}

New-Item -Path $OutputDir -ItemType Directory -ErrorAction Ignore | Out-Null

foreach ($InputFile in $InputFiles) {
    Get-Item $InputFile | Where-Object {
        -not $_.PSIsContainer
    } | Sort-Object | ForEach-Object {
        $InputFileName = $_.FullName
        $OutputFileName = Join-Path -Path $OutputDir -ChildPath $_.Name -ErrorAction Stop
        Write-Host "> faithtype -o ""$OutputFileName"" -- ""$InputFileName"""
        & $FaithTypeCmd $FaithTypeArgs '-o' $OutputFileName '--' $InputFileName
        Write-Host
    }
}
