#!/usr/bin/env pwsh

<# .SYNOPSIS #>

param (
    [String]
    [Parameter(Mandatory = $true)]
    # Directory that contains already patched fonts.
    $PatchedFontsDir = 'C:\Patched Fonts'
)

Set-StrictMode -Version 3.0

$RegistryKey = Get-Item -Path 'Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts'
foreach ($FontName in $RegistryKey.Property | Sort-Object) {
    $FontPath = $RegistryKey.GetValue($FontName)
    if ($FontPath -like '*[/\]*') {
        continue
    }
    $PatchedFontPath = Join-Path -Path $PatchedFontsDir -ChildPath $FontPath
    if (-not (Test-Path -Path $PatchedFontPath -PathType Leaf)) {
        continue
    }
    Write-Host "Installing: $PatchedFontPath"
    try {
        Set-ItemProperty -Path 'Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Fonts' -Name $FontName -Value $PatchedFontPath -Type String -ErrorAction Stop
    } catch [System.SystemException] {
        Write-Error -Exception $_.Exception -Message 'Failed to set registry value. Try run this script as administrator.'
        exit 1
    }
}

$RegistryKey = Get-Item -Path 'Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\FontLink\SystemLink'
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
        if ($FallbackFontPath -like '*[/\]*') {
            return $_
        }
        $PatchedFontPath = Join-Path -Path $PatchedFontsDir -ChildPath $FallbackFontPath
        if (-not (Test-Path -Path $PatchedFontPath -PathType Leaf)) {
            return $_
        }
        if ($PatchedFontPath.Contains(',')) {
            Write-Warning -Message "Patched font path contains comma, which is not supported by Windows font fallback system. Ignoring: $PatchedFontPath"
            return $_
        }
        $FallbackFontsChanged = $true
        if ($null -ne $FallbackFontName) {
            $PatchedFontPath.ToUpperInvariant() + ',' + $FallbackFontName
        } else {
            $PatchedFontPath.ToUpperInvariant()
        }
    }
    if ($FallbackFontsChanged) {
        Write-Host "Modifying font fallback: $FontName"
        try {
            Set-ItemProperty -Path 'Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\FontLink\SystemLink' -Name $FontName -Value $FallbackFonts -Type MultiString -ErrorAction Stop
        } catch [System.SystemException] {
            Write-Error -Exception $_.Exception -Message 'Failed to set registry value. Try run this script as administrator.'
            exit 1
        }
    }
}