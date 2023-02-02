lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Estat"),
        ("Your Desktop", "EL teu escriptori"),
        ("desk_tip", "Pots accedir al teu escriptori amb aquest ID i contrasenya."),
        ("Password", "Contrasenya"),
        ("Ready", "Llest"),
        ("Established", "Establert"),
        ("connecting_status", "Connexió a la xarxa HiDesk en progrés..."),
        ("Enable Service", "Habilitar Servei"),
        ("Start Service", "Iniciar Servei"),
        ("Service is running", "El servei s'està executant"),
        ("Service is not running", "El servei no s'està executant"),
        ("not_ready_status", "No està llest. Comprova la teva connexió"),
        ("Control Remote Desktop", "Controlar escriptori remot"),
        ("Transfer File", "Transferir arxiu"),
        ("Connect", "Connectar"),
        ("Recent Sessions", "Sessions recents"),
        ("Address Book", "Directori"),
        ("Confirmation", "Confirmació"),
        ("TCP Tunneling", "Túnel TCP"),
        ("Remove", "Eliminar"),
        ("Refresh random password", "Actualitzar contrasenya aleatòria"),
        ("Set your own password", "Estableix la teva pròpia contrasenya"),
        ("Enable Keyboard/Mouse", "Habilitar teclat/ratolí"),
        ("Enable Clipboard", "Habilitar portapapers"),
        ("Enable File Transfer", "Habilitar transferència d'arxius"),
        ("Enable TCP Tunneling", "Habilitar túnel TCP"),
        ("IP Whitelisting", "Direccions IP admeses"),
        ("ID/Relay Server", "Servidor ID/Relay"),
        ("Import Server Config", "Importar configuració de servidor"),
        ("Export Server Config", "Exportar configuració del servidor"),
        ("Import server configuration successfully", "Configuració de servidor importada amb èxit"),
        ("Export server configuration successfully", "Configuració de servidor exportada con èxit"),
        ("Invalid server configuration", "Configuració de servidor incorrecta"),
        ("Clipboard is empty", "El portapapers està buit"),
        ("Stop service", "Aturar servei"),
        ("Change ID", "Canviar ID"),
        ("Website", "Lloc web"),
        ("About", "Sobre"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Silenciar"),
        ("Audio Input", "Entrada d'àudio"),
        ("Enhancements", "Millores"),
        ("Hardware Codec", "Còdec de hardware"),
        ("Adaptive Bitrate", "Tasa de bits adaptativa"),
        ("ID Server", "Servidor de IDs"),
        ("Relay Server", "Servidor Relay"),
        ("API Server", "Servidor API"),
        ("invalid_http", "ha de començar amb http:// o https://"),
        ("Invalid IP", "IP incorrecta"),
        ("id_change_tip", "Només pots utilitzar caràcters a-z, A-Z, 0-9 e _ (guionet baix). El primer caràcter ha de ser a-z o A-Z. La longitut ha d'estar entre 6 i 16 caràcters."),
        ("Invalid format", "Format incorrecte"),
        ("server_not_support", "Encara no és compatible amb el servidor"),
        ("Not available", "No disponible"),
        ("Too frequent", "Massa comú"),
        ("Cancel", "Cancel·lar"),
        ("Skip", "Saltar"),
        ("Close", "Tancar"),
        ("Retry", "Reintentar"),
        ("OK", ""),
        ("Password Required", "Es necessita la contrasenya"),
        ("Please enter your password", "Si us plau, introdueixi la seva contrasenya"),
        ("Remember password", "Recordar contrasenya"),
        ("Wrong Password", "Contrasenya incorrecta"),
        ("Do you want to enter again?", "Vol tornar a entrar?"),
        ("Connection Error", "Error de connexió"),
        ("Error", ""),
        ("Reset by the peer", "Reestablert pel peer"),
        ("Connecting...", "Connectant..."),
        ("Connection in progress. Please wait.", "Connexió en procés. Esperi."),
        ("Please try 1 minute later", "Torni a provar-ho d'aquí un minut"),
        ("Login Error", "Error d'inicio de sessió"),
        ("Successful", "Exitós"),
        ("Connected, waiting for image...", "Connectant, esperant imatge..."),
        ("Name", "Nom"),
        ("Type", "Tipus"),
        ("Modified", "Modificat"),
        ("Size", "Grandària"),
        ("Show Hidden Files", "Mostrar arxius ocults"),
        ("Receive", "Rebre"),
        ("Send", "Enviar"),
        ("Refresh File", "Actualitzar arxiu"),
        ("Local", ""),
        ("Remote", "Remot"),
        ("Remote Computer", "Ordinador remot"),
        ("Local Computer", "Ordinador local"),
        ("Confirm Delete", "Confirma eliminació"),
        ("Delete", "Eliminar"),
        ("Properties", "Propietats"),
        ("Multi Select", "Selecció múltiple"),
        ("Select All", "Selecciona-ho Tot"),
        ("Unselect All", "Deselecciona-ho Tot"),
        ("Empty Directory", "Directori buit"),
        ("Not an empty directory", "No és un directori buit"),
        ("Are you sure you want to delete this file?", "Estàs segur que vols eliminar aquest arxiu?"),
        ("Are you sure you want to delete this empty directory?", "Estàs segur que vols eliminar aquest directori buit?"),
        ("Are you sure you want to delete the file of this directory?", "Estàs segur que vols eliminar aquest arxiu d'aquest directori?"),
        ("Do this for all conflicts", "Fes això per a tots els conflictes"),
        ("This is irreversible!", "Això és irreversible!"),
        ("Deleting", "Eliminant"),
        ("files", "arxius"),
        ("Waiting", "Esperant"),
        ("Finished", "Acabat"),
        ("Speed", "Velocitat"),
        ("Custom Image Quality", "Qualitat d'imatge personalitzada"),
        ("Privacy mode", "Mode privat"),
        ("Block user input", "Bloquejar entrada d'usuari"),
        ("Unblock user input", "Desbloquejar entrada d'usuari"),
        ("Adjust Window", "Ajustar finestra"),
        ("Original", "Original"),
        ("Shrink", "Reduir"),
        ("Stretch", "Estirar"),
        ("Scrollbar", "Barra de desplaçament"),
        ("ScrollAuto", "Desplaçament automàtico"),
        ("Good image quality", "Bona qualitat d'imatge"),
        ("Balanced", "Equilibrat"),
        ("Optimize reaction time", "Optimitzar el temps de reacció"),
        ("Custom", "Personalitzat"),
        ("Show remote cursor", "Mostrar cursor remot"),
        ("Show quality monitor", "Mostrar qualitat del monitor"),
        ("Disable clipboard", "Deshabilitar portapapers"),
        ("Lock after session end", "Bloquejar després del final de la sessió"),
        ("Insert", "Inserir"),
        ("Insert Lock", "Inserir bloqueig"),
        ("Refresh", "Actualitzar"),
        ("ID does not exist", "L'ID no existeix"),
        ("Failed to connect to rendezvous server", "No es pot connectar al servidor rendezvous"),
        ("Please try later", "Siusplau provi-ho més tard"),
        ("Remote desktop is offline", "L'escriptori remot està desconecctat"),
        ("Key mismatch", "La clau no coincideix"),
        ("Timeout", "Temps esgotat"),
        ("Failed to connect to relay server", "No es pot connectar al servidor de relay"),
        ("Failed to connect via rendezvous server", "No es pot connectar a través del servidor de rendezvous"),
        ("Failed to connect via relay server", "No es pot connectar a través del servidor de relay"),
        ("Failed to make direct connection to remote desktop", "No s'ha pogut establir una connexió directa amb l'escriptori remot"),
        ("Set Password", "Configurar la contrasenya"),
        ("OS Password", "contrasenya del sistema operatiu"),
        ("install_tip", ""),
        ("Click to upgrade", "Clicar per actualitzar"),
        ("Click to download", "Clicar per descarregar"),
        ("Click to update", "Clicar per refrescar"),
        ("Configure", "Configurar"),
        ("config_acc", ""),
        ("config_screen", ""),
        ("Installing ...", "Instal·lant ..."),
        ("Install", "Instal·lar"),
        ("Installation", "Instal·lació"),
        ("Installation Path", "Ruta d'instal·lació"),
        ("Create start menu shortcuts", "Crear accessos directes al menú d'inici"),
        ("Create desktop icon", "Crear icona d'escriptori"),
        ("agreement_tip", ""),
        ("Accept and Install", "Acceptar i instal·lar"),
        ("End-user license agreement", "Acord de llicència d'usuario final"),
        ("Generating ...", "Generant ..."),
        ("Your installation is lower version.", "La seva instal·lació és una versión inferior."),
        ("not_close_tcp_tip", ""),
        ("Listening ...", "Escoltant..."),
        ("Remote Host", "Hoste remot"),
        ("Remote Port", "Port remot"),
        ("Action", "Acció"),
        ("Add", "Afegirr"),
        ("Local Port", "Port local"),
        ("Local Address", "Adreça Local"),
        ("Change Local Port", "Canviar Port Local"),
        ("setup_server_tip", ""),
        ("Too short, at least 6 characters.", "Massa curt, almenys 6 caràcters."),
        ("The confirmation is not identical.", "La confirmación no coincideix."),
        ("Permissions", "Permisos"),
        ("Accept", "Acceptar"),
        ("Dismiss", "Cancel·lar"),
        ("Disconnect", "Desconnectar"),
        ("Allow using keyboard and mouse", "Permetre l'ús del teclat i ratolí"),
        ("Allow using clipboard", "Permetre usar portapapers"),
        ("Allow hearing sound", "Permetre escoltar so"),
        ("Allow file copy and paste", "Permetre copiar i enganxar arxius"),
        ("Connected", "Connectat"),
        ("Direct and encrypted connection", "Connexió directa i xifrada"),
        ("Relayed and encrypted connection", "connexió retransmesa i xifrada"),
        ("Direct and unencrypted connection", "connexió directa i sense xifrar"),
        ("Relayed and unencrypted connection", "connexió retransmesa i sense xifrar"),
        ("Enter Remote ID", "Introduixi l'ID remot"),
        ("Enter your password", "Introdueixi la seva contrasenya"),
        ("Logging in...", "Iniciant sessió..."),
        ("Enable RDP session sharing", "Habilitar l'ús compartit de sessions RDP"),
        ("Auto Login", "Inici de sessió automàtic"),
        ("Enable Direct IP Access", "Habilitar accés IP directe"),
        ("Rename", "Renombrar"),
        ("Space", "Espai"),
        ("Create Desktop Shortcut", "Crear accés directe a l'escriptori"),
        ("Change Path", "Cnviar ruta"),
        ("Create Folder", "Crear carpeta"),
        ("Please enter the folder name", "Indiqui el nom de la carpeta"),
        ("Fix it", "Soluciona-ho"),
        ("Warning", "Avís"),
        ("Login screen using Wayland is not supported", "La pantalla d'inici de sessió amb Wayland no és compatible"),
        ("Reboot required", "Cal reiniciar"),
        ("Unsupported display server ", "Servidor de visualització no compatible"),
        ("x11 expected", "x11 necessari"),
        ("Port", ""),
        ("Settings", "Ajustaments"),
        ("Username", " Nom d'usuari"),
        ("Invalid port", "Port incorrecte"),
        ("Closed manually by the peer", "Tancat manualment pel peer"),
        ("Enable remote configuration modification", "Habilitar modificació remota de configuració"),
        ("Run without install", "Executar sense instal·lar"),
        ("Always connected via relay", "Connectat sempre a través de relay"),
        ("Always connect via relay", "Connecta sempre a través de relay"),
        ("whitelist_tip", ""),
        ("Login", "Inicia sessió"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "Sortir"),
        ("Tags", ""),
        ("Search ID", "Cerca ID"),
        ("Current Wayland display server is not supported", "El servidor de visualització actual de Wayland no és compatible"),
        ("whitelist_sep", ""),
        ("Add ID", "Afegir ID"),
        ("Add Tag", "Afegir tag"),
        ("Unselect all tags", "Deseleccionar tots els tags"),
        ("Network error", "Error de xarxa"),
        ("Username missed", "Nom d'usuari oblidat"),
        ("Password missed", "Contrasenya oblidada"),
        ("Wrong credentials", "Credencials incorrectes"),
        ("Edit Tag", "Editar tag"),
        ("Unremember Password", "Contrasenya oblidada"),
        ("Favorites", "Preferits"),
        ("Add to Favorites", "Afegir a preferits"),
        ("Remove from Favorites", "Treure de preferits"),
        ("Empty", "Buit"),
        ("Invalid folder name", "Nom de carpeta incorrecte"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Hostname", ""),
        ("Discovered", "Descobert"),
        ("install_daemon_tip", ""),
        ("Remote ID", "ID remot"),
        ("Paste", "Enganxar"),
        ("Paste here?", "Enganxar aquí?"),
        ("Are you sure to close the connection?", "Estàs segur que vols tancar la connexió?"),
        ("Download new version", "Descarregar nova versió"),
        ("Touch mode", "Mode tàctil"),
        ("Mouse mode", "Mode ratolí"),
        ("One-Finger Tap", "Toqui amb un dit"),
        ("Left Mouse", "Ratolí esquerra"),
        ("One-Long Tap", "Toc llarg"),
        ("Two-Finger Tap", "Toqui amb dos dits"),
        ("Right Mouse", "Botó dret"),
        ("One-Finger Move", "Moviment amb un dir"),
        ("Double Tap & Move", "Toqui dos cops i mogui"),
        ("Mouse Drag", "Arrastri amb el ratolí"),
        ("Three-Finger vertically", "Tres dits verticalment"),
        ("Mouse Wheel", "Roda del ratolí"),
        ("Two-Finger Move", "Moviment amb dos dits"),
        ("Canvas Move", "Moviment del llenç"),
        ("Pinch to Zoom", "Pessiga per fer zoom"),
        ("Canvas Zoom", "Ampliar llenç"),
        ("Reset canvas", "Reestablir llenç"),
        ("No permission of file transfer", "No tens permís de transferència de fitxers"),
        ("Note", "Nota"),
        ("Connection", "connexió"),
        ("Share Screen", "Compartir pantalla"),
        ("CLOSE", "TANCAR"),
        ("OPEN", "OBRIR"),
        ("Chat", "Xat"),
        ("Total", "Total"),
        ("items", "ítems"),
        ("Selected", "Seleccionat"),
        ("Screen Capture", "Captura de pantalla"),
        ("Input Control", "Control d'entrada"),
        ("Audio Capture", "Captura d'àudio"),
        ("File Connection", "connexió d'arxius"),
        ("Screen Connection", "connexió de pantalla"),
        ("Do you accept?", "Acceptes?"),
        ("Open System Setting", "Configuració del sistema obert"),
        ("How to get Android input permission?", "Com obtenir el permís d'entrada d'Android?"),
        ("android_input_permission_tip1", "Per a que un dispositiu remot controli el seu dispositiu Android amb el ratolí o tocs, cal permetre que HiDesk utilitzi el servei d' \"Accesibilitat\"."),
        ("android_input_permission_tip2", "Vagi a la pàgina de [Serveis instal·lats], activi el servici [HiDesk Input]."),
        ("android_new_connection_tip", "S'ha rebut una nova sol·licitud de control per al dispositiu actual."),
        ("android_service_will_start_tip", "Habilitar la captura de pantalla iniciarà el servei automàticament, i permetrà que altres dispositius sol·licitin una connexió des d'aquest dispositiu."),
        ("android_stop_service_tip", "Tancar el servei tancarà totes les connexions establertes."),
        ("android_version_audio_tip", "La versión actual de Android no admet la captura d'àudio, actualizi a Android 10 o superior."),
        ("android_start_service_tip", "Toqui el permís [Iniciar servei] o OBRIR [Captura de pantalla] per iniciar el servei d'ús compartit de pantalla."),
        ("Account", "Compte"),
        ("Overwrite", "Sobreescriure"),
        ("This file exists, skip or overwrite this file?", "Aquest arxiu ja existeix, ometre o sobreescriure l'arxiu?"),
        ("Quit", "Sortir"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Ajuda"),
        ("Failed", "Ha fallat"),
        ("Succeeded", "Aconseguit"),
        ("Someone turns on privacy mode, exit", "Algú ha activat el mode de privacitat, surti"),
        ("Unsupported", "No suportat"),
        ("Peer denied", "Peer denegat"),
        ("Please install plugins", "Instal·li complements"),
        ("Peer exit", "El peer ha sortit"),
        ("Failed to turn off", "Error en apagar"),
        ("Turned off", "Apagat"),
        ("In privacy mode", "En mode de privacitat"),
        ("Out privacy mode", "Fora del mode de privacitat"),
        ("Language", "Idioma"),
        ("Keep HiDesk background service", "Mantenir HiDesk com a servei en segon pla"),
        ("Ignore Battery Optimizations", "Ignorar optimizacions de la bateria"),
        ("android_open_battery_optimizations_tip", ""),
        ("Connection not allowed", "Connexió no disponible"),
        ("Legacy mode", "Mode heretat"),
        ("Map mode", "Mode mapa"),
        ("Translate mode", "Mode traduit"),
        ("Use permanent password", "Utilitzar contrasenya permament"),
        ("Use both passwords", "Utilitzar ambdues contrasenyas"),
        ("Set permanent password", "Establir contrasenya permament"),
        ("Enable Remote Restart", "Activar reinici remot"),
        ("Allow remote restart", "Permetre reinici remot"),
        ("Restart Remote Device", "Reiniciar dispositiu"),
        ("Are you sure you want to restart", "Està segur que vol reiniciar?"),
        ("Restarting Remote Device", "Reiniciant dispositiu remot"),
        ("remote_restarting_tip", "Dispositiu remot reiniciant, tanqui aquest missatge i tornis a connectar amb la contrasenya."),
        ("Copied", "Copiat"),
        ("Exit Fullscreen", "Sortir de la pantalla completa"),
        ("Fullscreen", "Pantalla completa"),
        ("Mobile Actions", "Accions mòbils"),
        ("Select Monitor", "Seleccionar monitor"),
        ("Control Actions", "Accions de control"),
        ("Display Settings", "Configuració de pantalla"),
        ("Ratio", "Relació"),
        ("Image Quality", "Qualitat d'imatge"),
        ("Scroll Style", "Estil de desplaçament"),
        ("Show Menubar", "Mostra barra de menú"),
        ("Hide Menubar", "Amaga barra de menú"),
        ("Direct Connection", "Connexió directa"),
        ("Relay Connection", "Connexió Relay"),
        ("Secure Connection", "Connexió segura"),
        ("Insecure Connection", "Connexió insegura"),
        ("Scale original", "Escala original"),
        ("Scale adaptive", "Escala adaptativa"),
        ("General", ""),
        ("Security", "Seguretat"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tema Fosc"),
        ("Dark", "Fosc"),
        ("Light", "Clar"),
        ("Follow System", "Tema del sistema"),
        ("Enable hardware codec", "Habilitar còdec per hardware"),
        ("Unlock Security Settings", "Desbloquejar ajustaments de seguretat"),
        ("Enable Audio", "Habilitar àudio"),
        ("Unlock Network Settings", "Desbloquejar Ajustaments de Xarxa"),
        ("Server", "Servidor"),
        ("Direct IP Access", "Accés IP Directe"),
        ("Proxy", ""),
        ("Apply", "Aplicar"),
        ("Disconnect all devices?", "Desconnectar tots els dispositius?"),
        ("Clear", "Netejar"),
        ("Audio Input Device", "Dispositiu d'entrada d'àudio"),
        ("Deny remote access", "Denegar accés remot"),
        ("Use IP Whitelisting", "Utilitza llista de IPs admeses"),
        ("Network", "Xarxa"),
        ("Enable RDP", "Habilitar RDP"),
        ("Pin menubar", "Bloqueja barra de menú"),
        ("Unpin menubar", "Desbloquejar barra de menú"),
        ("Recording", "Gravant"),
        ("Directory", "Directori"),
        ("Automatically record incoming sessions", "Gravació automàtica de sessions entrants"),
        ("Change", "Canviar"),
        ("Start session recording", "Començar gravació de sessió"),
        ("Stop session recording", "Aturar gravació de sessió"),
        ("Enable Recording Session", "Habilitar gravació de sessió"),
        ("Allow recording session", "Permetre gravació de sessió"),
        ("Enable LAN Discovery", "Habilitar descobriment de LAN"),
        ("Deny LAN Discovery", "Denegar descobriment de LAN"),
        ("Write a message", "Escriure un missatge"),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", "Desconnectat"),
        ("Other", "Altre"),
        ("Confirm before closing multiple tabs", "Confirmar abans de tancar múltiples pestanyes"),
        ("Keyboard Settings", "Ajustaments de teclat"),
        ("Full Access", "Acces complet"),
        ("Screen Share", "Compartir pantalla"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland requereix Ubuntu 21.04 o una versió superior."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland requereix una versió superior de la distribución de Linux. Provi l'escriptori X11 o canvïi el seu sistema operatiu."),
        ("JumpLink", "Veure"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Seleccioni la pantalla que es compartirà (Operar al costat del peer)."),
        ("Show HiDesk", "Mostrar HiDesk"),
        ("This PC", "Aquest PC"),
        ("or", "o"),
        ("Continue with", "Continuar amb"),
        ("Elevate", ""),
        ("Zoom cursor", ""),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", ""),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
        ("Hide connection management window", ""),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to Address Book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by the web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
        ("config_input", ""),
        ("request_elevation_tip", ""),
        ("Wait", ""),
        ("Elevation Error", ""),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", ""),
        ("Request Elevation", ""),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
    ].iter().cloned().collect();
}
