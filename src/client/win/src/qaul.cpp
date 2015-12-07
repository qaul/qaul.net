/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#include "stdafx.h"
#include "formStart.h"

using namespace qaul;

[STAThreadAttribute]
int main(array<System::String ^> ^args)
{
	// activate visual effects on Windows XP, before control elements are created 
	Application::EnableVisualStyles();
	Application::SetCompatibleTextRenderingDefault(false); 

	// create main window and start
	Application::Run(gcnew formStart());
	return 0;
}
