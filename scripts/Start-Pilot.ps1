param([switch]$Test, [switch]$Bootstrap)
$ErrorActionPreference = 'Stop'
$taskRoot = Split-Path $PSScriptRoot -Parent
Push-Location $taskRoot
try {
    $taskPrivate = Join-Path $taskRoot '.pilot-private'
    New-Item -ItemType Directory -Force -Path $taskPrivate | Out-Null
    $taskEnvFile = Join-Path $taskPrivate 'local.env'
    if (!(Test-Path -LiteralPath $taskEnvFile)) {
        $taskRng = [Security.Cryptography.RandomNumberGenerator]::Create()
        $taskBytes = New-Object byte[] 48
        $taskRng.GetBytes($taskBytes)
        $taskDbSecret = [BitConverter]::ToString($taskBytes).Replace('-', '')
        $taskRng.GetBytes($taskBytes)
        $taskJwtSecret = [BitConverter]::ToString($taskBytes).Replace('-', '')
        $taskRng.Dispose()
        [IO.File]::WriteAllText($taskEnvFile, "POSTGRES_PASSWORD=$taskDbSecret`nJWT_SECRET=$taskJwtSecret`n")
    }
    $taskCompose = @('compose', '--env-file', $taskEnvFile, '-f', 'compose.pilot.yml')
    & docker @taskCompose up -d db
    if ($LASTEXITCODE -ne 0) { throw 'No se pudo iniciar PostgreSQL. Abre Docker Desktop.' }
    if ($Test) {
        & docker @taskCompose run --rm api cargo test --locked
        if ($LASTEXITCODE -ne 0) { throw 'Fallaron las pruebas del piloto.' }
    } elseif ($Bootstrap) {
        $env:BOOTSTRAP_ADMIN_EMAIL = Read-Host 'Correo del primer operador'
        $taskPassword = Read-Host 'Contraseña única de al menos 16 caracteres' -AsSecureString
        $env:BOOTSTRAP_ADMIN_PASSWORD = [Net.NetworkCredential]::new('', $taskPassword).Password
        try {
            & docker @taskCompose run --rm -e BOOTSTRAP_ADMIN_EMAIL -e BOOTSTRAP_ADMIN_PASSWORD api cargo run --locked -- bootstrap-admin
            if ($LASTEXITCODE -ne 0) { throw 'No se pudo crear el operador. Revisa el mensaje anterior.' }
        } finally {
            Remove-Item Env:BOOTSTRAP_ADMIN_EMAIL, Env:BOOTSTRAP_ADMIN_PASSWORD -ErrorAction SilentlyContinue
        }
    } else {
        & docker @taskCompose up -d api
        if ($LASTEXITCODE -ne 0) { throw 'No se pudo iniciar la API.' }
        Write-Host 'Piloto: http://localhost:3005/pilot (la primera compilación tarda varios minutos).'
    }
} finally { Pop-Location }
