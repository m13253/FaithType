#!/usr/bin/env pwsh

<# .SYNOPSIS #>

param (
    [String]
    # Windows stock font directory.
    $StockFontsDir = 'C:\Windows\Fonts',

    [String]
    # Path to output files.
    $PatchedFontsDir = 'C:\Windows\Fonts\FaithType',

    [String]
    $_Private0 = '',
    [String]
    $_Private1 = ''
)

Set-StrictMode -Version 3.0

if ($_Private0 -eq '') {

    $_Private0 = (Get-Location).Path
    $_Private1 = if ($null -ne $Env:CARGO -and $Env:CARGO -ne '') {
        $Env:CARGO
    } else {
        ''
    }
    Start-Process -FilePath 'powershell.exe' -ArgumentList (
        '-ExecutionPolicy', 'Bypass', '-NoLogo', '-NoProfile', '-File', """$($PSCommandPath.replace('"', '\"'))""", '-StockFontsDir', """$($StockFontsDir.replace('"', '\"'))""", '-PatchedFontsDir', """$($PatchedFontsDir.replace('"', '\"'))""", '-_Private0', """$($_Private0.replace('"', '\"'))""", '-_Private1', """$($_Private1.replace('"', '\"'))"""
    ) -Verb 'RunAs' -ErrorAction Stop | Out-Null

} else {

    try {
        $FilesToPatch = (
            # Arial (Latin)
            'arialbd.ttf',
            'arialbi.ttf',
            'ariali.ttf',
            'arial.ttf',
            'ariblk.ttf',
            # Batang (Korean)
            'batang.ttc',
            # Calibri (Latin)
            'calibrib.ttf',
            'calibrii.ttf',
            'calibrili.ttf',
            'calibril.ttf',
            'calibri.ttf',
            'calibriz.ttf',
            # Cambria (Latin)
            'cambriab.ttf',
            'cambriai.ttf',
            'cambria.ttc',
            'cambriaz.ttf',
            # Candara (Latin)
            'Candarab.ttf',
            'Candarai.ttf',
            'Candarali.ttf',
            'Candaral.ttf',
            'Candara.ttf',
            'Candaraz.ttf',
            # Comic Sans MS (Latin)
            'comicbd.ttf',
            'comici.ttf',
            'comic.ttf',
            'comicz.ttf',
            # Consolas (Latin)
            'consolab.ttf',
            'consolai.ttf',
            'consola.ttf',
            'consolaz.ttf',
            # Constantia (Latin)
            'constanb.ttf',
            'constani.ttf',
            'constan.ttf',
            'constanz.ttf',
            # Corbel (Latin)
            'corbelb.ttf',
            'corbeli.ttf',
            'corbelli.ttf',
            'corbell.ttf',
            'corbel.ttf',
            'corbelz.ttf',
            # Courier New (Latin)
            'couri.ttf',
            'cour.ttf',
            # DengXian (Simplified Chinese)
            'Dengb.ttf',
            'Dengl.ttf',
            'Deng.ttf',
            # Georgia (Latin)
            'georgiab.ttf',
            'georgiai.ttf',
            'georgia.ttf',
            'georgiaz.ttf',
            # Gulim (Korean)
            'gulim.ttc',
            # Impact (Latin)
            'impact.ttf',
            # Lucida Sans Unicode (Latin)
            'l_10646.ttf',
            # Lucida Console (Latin)
            'lucon.ttf',
            # Malgun Gothic (Korean)
            'malgunbd.ttf',
            'malgunsl.ttf',
            'malgun.ttf',
            # Meiryo (Japanese)
            'meiryob.ttc',
            'meiryo.ttc',
            # Microsoft Sans Serif (Latin)
            'micross.ttf',
            # MingLiU (Traditional Chinese)
            'mingliub.ttc',
            'mingliu.ttc',
            # MS Gothic (Japanese)
            'msgothic.ttc',
            # Microsoft JhengHei (Traditional Chinese)
            'msjhbd.ttc',
            'msjhl.ttc',
            'msjh.ttc',
            # MS Mincho (Japanese)
            'msmincho.ttc',
            # Microsoft YaHei (Simplified Chinese)
            'msyhbd.ttc',
            'msyhl.ttc',
            'msyh.ttc',
            # Segoe MDL2 Assets (Symbols)
            'segmdl2.ttf',
            # Segoe UI (Latin)
            'segoeuib.ttf',
            'segoeuii.ttf',
            'segoeuil.ttf',
            'segoeuisl.ttf',
            'segoeui.ttf',
            'segoeuiz.ttf',
            'seguibli.ttf',
            'seguibl.ttf',
            'seguili.ttf',
            'seguisbi.ttf',
            'seguisb.ttf',
            'seguisli.ttf',
            # Segoe UI Symbol (Symbols)
            'seguisym.ttf',
            # Segoe UI Variable (Latin)
            'SegUIVar.ttf',
            # FangSong (Simplified Chinese)
            'simfang.ttf',
            # SimHei (Simpified Chinese)
            'simhei.ttf',
            # KaiTi (Simplified Chinese)
            'simkai.ttf',
            # SimSun (Simplified Chinese)
            'simsunb.ttf',
            'simsun.ttc',
            # Symbol (Symbols)
            'symbol.ttf',
            # Tahoma (Latin)
            'tahomabd.ttf',
            'tahoma.ttf',
            # Times New Roman (Latin)
            'timesbd.ttf',
            'timesbi.ttf',
            'timesi.ttf',
            'times.ttf',
            # Trebuchet MS (Latin)
            'trebucbd.ttf',
            'trebucbi.ttf',
            'trebucit.ttf',
            'trebuc.ttf',
            # Verdana (Latin)
            'verdanab.ttf',
            'verdanai.ttf',
            'verdana.ttf',
            'verdanaz.ttf',
            # Webdings (Symbols)
            'webdings.ttf',
            # Wingdings (Symbols)
            'wingding.ttf',
            # Yu Gothic (Japanese)
            'YuGothB.ttc',
            'YuGothL.ttc',
            'YuGothM.ttc',
            'YuGothR.ttc',
            # Yu Mincho (Japanese)
            'yumindb.ttf',
            'yuminl.ttf',
            'yumin.ttf'
        )

        Set-Location -LiteralPath $_Private0 -ErrorAction Stop
        if ($_Private1 -ne '') {
            $Env:CARGO = $_Private1
        }
        $InputPaths = $FilesToPatch | ForEach-Object {
            Join-Path -Path $StockFontsDir -ChildPath $_ -ErrorAction Stop
        } | Where-Object {
            Test-Path $_ -PathType Leaf -ErrorAction Stop
        }
        . $PSScriptRoot\Manual-Batch-Patch.ps1 -OutputDir $PatchedFontsDir -InputFiles $InputPaths 
        . $PSScriptRoot\Manual-Install-Registry.ps1 -PatchedFontsDir $PatchedFontsDir
    } catch {
        Write-Error -Exception $_.Exception
    }

    Write-Host
    Read-Host 'Press Enter to exit'
}
