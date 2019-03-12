/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * This file is a dummy implementation without the presence of a VOIP client.
 * This version is used if no VOIP shall be present. Otherwise the file
 * qaullib_voip.c will be included
 */

#include "qaullib_private.h"

void Qaullib_VoipCallStart(char* ip)
{
	printf("Qaullib_VoipCallStart()\n");
}

void Qaullib_VoipCallAccept(void)
{
	printf("Qaullib_VoipCallAccept()\n");
}

void Qaullib_VoipCallEnd(void)
{
	printf("Qaullib_VoipCallEnd()\n");
}

int Qaullib_VoipStart(void)
{
	printf("Qaullib_VoipStart()\n");
    return 1;
}

void Qaullib_VoipStop(void)
{
	printf("Qaullib_VoipStop()\n");
}
