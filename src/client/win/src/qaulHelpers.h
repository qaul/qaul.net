/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#define qaulWifiProfile "<?xml version=\"1.0\"?><WLANProfile xmlns=\"http://www.microsoft.com/networking/WLAN/profile/v1\"><name>qaul.net</name><SSIDConfig><SSID><hex>7161756C2E6E6574</hex><name>qaul.net</name></SSID><nonBroadcast>false</nonBroadcast></SSIDConfig><connectionType>IBSS</connectionType><connectionMode>manual</connectionMode><autoSwitch>false</autoSwitch><MSM><security><authEncryption><authentication>open</authentication><encryption>none</encryption><useOneX>false</useOneX></authEncryption></security></MSM></WLANProfile>"


namespace qaul
{
	//LPCWSTR lpcwstrProfile = _T("<?xml version=\"1.0\"?><WLANProfile xmlns=\"http://www.microsoft.com/networking/WLAN/profile/v1\"><name>qaul.net</name><SSIDConfig><SSID><hex>7161756C2E6E6574</hex><name>qaul.net</name></SSID><nonBroadcast>false</nonBroadcast></SSIDConfig><connectionType>IBSS</connectionType><connectionMode>manual</connectionMode><autoSwitch>false</autoSwitch><MSM><security><authEncryption><authentication>open</authentication><encryption>none</encryption><useOneX>false</useOneX></authEncryption></security></MSM></WLANProfile>");


	public class qaulInterface
	{
	public:
		// Interface
		bool  InterfaceFound;
		DWORD InterfaceIndex;
		GUID  InterfaceGuid;
		char  InterfaceName[256 +4]; // Friendly Name

		// olsrd
		STARTUPINFO StartInfo;
		PROCESS_INFORMATION ProcInfo;

		qaulInterface()
		{
			InterfaceFound = false;
		}
	};


}