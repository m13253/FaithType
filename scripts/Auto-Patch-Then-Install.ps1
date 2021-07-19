#!/usr/bin/env pwsh

<# .SYNOPSIS #>

param (
    [String]
    # Windows stock font directory.
    $StockFontsDir = 'C:\Windows\Fonts',

    [String]
    # Path to output files.
    $PatchedFontsDir = 'C:\Patched Fonts'
)

Set-StrictMode -Version 3.0

$FilesToPatch = (
    # Arial (Latin)
    'arialbd.ttf',
    'arialbi.ttf',
    'ariali.ttf',
    'arial.ttf',
    'ariblk.ttf',
    # Batang (Korean)
    'batang.ttc',
    # Consolas (Latin)
    'consolab.ttf',
    'consolai.ttf',
    'consola.ttf',
    'consolaz.ttf',
    # Courier New (Latin)
    'couri.ttf',
    'cour.ttf',
    # Gulim (Korean)
    'gulim.ttc',
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
    # Fangsong (Simplified Chinese)
    'simfang.ttf',
    # SimHei (Simpified Chinese)
    'simhei.ttf',
    # KaiTi (Simplified Chinese)
    'simkai.ttf',
    # SimSun (Simplified Chinese)
    'simsunb.ttf',
    'simsun.ttc',
    # Tahoma (Latin)
    'tahomabd.ttf',
    'tahoma.ttf',
    # Times New Roman (Latin)
    'timesbd.ttf',
    'timesbi.ttf',
    'timesi.ttf',
    'times.ttf',
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

$InputPaths = $FilesToPatch | ForEach-Object {
    Join-Path -Path $StockFontsDir -ChildPath $_
} | Where-Object {
    Test-Path $_ -PathType Leaf
}
.\Manual-Batch-Patch.ps1 -OutputDir $PatchedFontsDir -InputFiles $InputPaths 
.\Install-Registry.ps1 -PatchedFontsDir $PatchedFontsDir
