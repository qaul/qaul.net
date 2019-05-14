qaul Headless Daemon 'qauld'
============================

On Linux, the executable 'qauld', the headless qaul.net daemon is built.
qauld has some CLI options to configure qaul.net from the command line.

Options
-------

--ip <IPv4>, -i <IPv4>
  Set the device IP address version 4. Usage example: -i 10.111.209.12
--username <USERNAME>, -u <USERNAME>
  Set device user name. Usage example: -u MyNickName
--locale <LOCALE>, -l <LOCALE>
  Set GUI locale. This is the default language for the admin GUI containing 
  two letters. The web users can configure their own language. 
  The following languages exist: 
  en (English), de (German), cn (Chinese), ar (Arabic), es (Spanish), 
  he (Greek)
  Usage example: -l en
--interface <NETWORK_INTERFACE>, -n <NETWORK_INTERFACE>
  Set network interface name to use. e.g.: -n wlan0
--download <AUTO_DOWNLOAD>, -d <AUTO_DOWNLOAD>
  configure whether files form the file sharing shall be automatically 
  downloaded.
  Usage example: -d 1
--max_storage <MAX_STORAGE_BYTE_SIZE>, -s <MAX_STORAG_BYTE_SIZE>
  When auto-downloading files, one should configure how big the maximal
  storage size on the device is allowed to become. When the maximal 
  storage size is reached, qaul will start deleting old and not often 
  used files from storage to make space for newly shared files.
  Usage example (10GB): -s 10000000000
  

minimal setup for first start
-----------------------------

At first start, qauld needs to be configured. It needs at least a user
name and a locale.

example: 

    qauld -u MyUserName -l es

