#!/usr/bin/env pwsh

<# .SYNOPSIS #>

param (
    [String]
    # Windows stock font directory.
    $StockFontsDir = 'C:\Windows\Fonts',

    [String]
    $_Private0 = ''
)

Set-StrictMode -Version 3.0

if ($_Private0 -eq '') {

    $_Private0 = (Get-Location).Path
    Start-Process -FilePath 'powershell.exe' -ArgumentList (
        '-ExecutionPolicy', 'Bypass', '-NoLogo', '-NoProfile', '-File', """$($PSCommandPath.replace('"', '\"'))""", '-StockFontsDir', """$($StockFontsDir.replace('"', '\"'))""", '-_Private0', """$($_Private0.replace('"', '\"'))"""
    ) -Verb 'RunAs' -Wait -ErrorAction Stop | Out-Null

} else {

    try {
        Set-Location -LiteralPath $_Private0 -ErrorAction Stop
        . $PSScriptRoot\Uninstall-Registry.ps1 -StockFontsDir $StockFontsDir
    } catch {
        Write-Error -Exception $_.Exception
    }

    Write-Host
    Read-Host 'Press Enter to exit'
}
