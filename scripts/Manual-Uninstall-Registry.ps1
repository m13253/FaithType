#!/usr/bin/env pwsh

<# .SYNOPSIS #>

param (
    [String]
    # Windows stock font directory.
    $StockFontsDir = 'C:\Windows\Fonts'
)

Set-StrictMode -Version 3.0

Write-Host 'Unregistering fonts...'
$RegistryKey = Get-Item -Path 'Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts' -ErrorAction Stop
foreach ($FontName in $RegistryKey.Property | Sort-Object) {
    $FontPath = $RegistryKey.GetValue($FontName)
    if (-not ($FontPath -like '*[/\]*')) {
        continue
    }
    $FontFileName = $FontPath.Replace('/', '\').Split('\')[-1]
    $StockFontPath = Join-Path -Path $StockFontsDir -ChildPath $FontFileName -ErrorAction Stop
    if (-not (Test-Path -Path $StockFontPath -PathType Leaf -ErrorAction Stop)) {
        continue
    }
    Write-Host "Unregistering: $FontPath"
    try {
        Set-ItemProperty -Path 'Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts' -Name $FontName -Value $FontFileName -Type String -ErrorAction Stop
    } catch [System.SystemException] {
        Write-Error -Exception $_.Exception -Message 'Failed to set registry value. Try run this script as administrator.'
        exit 1
    }
}

Write-Host 'Restoring font fallback settings...'
$RegistryKey = Get-Item -Path 'Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\FontLink\SystemLink' -ErrorAction Stop
foreach ($FontName in $RegistryKey.Property | Sort-Object) {
    $FallbackFonts = $RegistryKey.GetValue($FontName)
    $FallbackFontsChanged = $false
    $FallbackFonts = $FallbackFonts | ForEach-Object {
        $FallbackFontSplit = $_.Split(',', 2)
        $FallbackFontPath = $FallbackFontSplit[0]
        $FallbackFontName = if ($FallbackFontSplit.Count -ge 2) {
            $FallbackFontSplit[1]
        } else {
            $null
        }
        if (-not ($FallbackFontPath -like '*[/\]*')) {
            return $_
        }
        $FallbackFontFileName = $FallbackFontPath.Replace('/', '\').Split('\')[-1]
        $StockFontPath = Join-Path -Path $StockFontsDir -ChildPath $FallbackFontFileName -ErrorAction Stop
        if (-not (Test-Path -Path $StockFontPath -PathType Leaf -ErrorAction Stop)) {
            return $_
        }
        $FallbackFontsChanged = $true
        if ($null -ne $FallbackFontName) {
            $FallbackFontFileName.ToUpperInvariant() + ',' + $FallbackFontName
        } else {
            $FallbackFontFileName.ToUpperInvariant()
        }
    }
    if ($FallbackFontsChanged) {
        Write-Host "Restoring: $FontName"
        try {
            Set-ItemProperty -Path 'Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\FontLink\SystemLink' -Name $FontName -Value $FallbackFonts -Type MultiString -ErrorAction Stop
        } catch [System.SystemException] {
            Write-Error -Exception $_.Exception -Message 'Failed to set registry value. Try run this script as administrator.'
            exit 1
        }
    }
}

Write-Host 'Fonts restored to stock version, please restart your device.'
