
$HOSTKTW = "https://repo.ecosolucionesweb.com/"

$PSStyle.Progress.View = "Minimal"

function descargarPaquetes {
    $total_paquetes = $paquetes.count
    $i = 1
    Write-Host "Descargando paquetes..." -Foreground "DarkCyan"

    foreach ($paquete in $paquetes) {
        $nombre, $res = $paquete -split "-"
        $win, $ver = $res -split "=="
            
        $url = "$HOSTKTW/repo/install/$nombre" + "?version=$ver&win=$win"
        
        Write-Host "Descargando el paquete $nombre..." -Foreground "DarkYellow"
    
        $destino = "./$nombre-$win-$ver.kt"
        try {
            $progressParams = @{
                Activity        = "Descargando $nombre"
                Status          = "Descargando..."
                PercentComplete = 0
            }
            
            $headResponse = Invoke-WebRequest -Uri $url -Method 'HEAD'
            [int]$fileSize = $headResponse.Headers['Content-Length'][0]

            $response = Invoke-WebRequest -Uri $url -Method 'GET' -OutFile $destino -ErrorAction Stop -PassThru
            $response | ForEach-Object {
                $progressParams.PercentComplete = [math]::Round(($_.BytesDownloaded / $fileSize) * 100)
                Write-Progress @progressParams
            }

            Write-Host "✅ Descarga completada" -Foreground "green" 
        }
        catch {
            Write-Host "✘ Error al descargar el paquete $nombre"  -Foreground "red"
        }

        [int]$porcentaje_paquetes = $i / $total_paquetes * 100
        Write-Progress -Activity "Descargando paquetes" -Status "$porcentaje_paquetes% Completado:" -PercentComplete $porcentaje_paquetes
        $i++
    }
    Write-Progress -Activity "Descargando paquetes" -Status "Descarga Completada" -Completed
}

function instalarPaquetes {
    Write-Host "Instalando paquetes..." -Foreground "DarkCyan"
}

function init {
    descargarPaquetes
    instalarPaquetes
}

init
