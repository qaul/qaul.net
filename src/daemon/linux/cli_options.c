/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <stdio.h> // defines FILENAME_MAX
#include <stdlib.h>
#include <getopt.h>
#include <string.h>

#include <qaullib.h>
#include "cli_options.h"
#include "qaul/utils/validate.h"

void qaul_cli_options(int argc, char *argv[])
{
	int c;
	char protected_username[MAX_USER_LEN +1];
	int max_storage_space;

	while (1)
	{
		static struct option long_options[] =
		{
			// These options set a flag.
			/*
			{"verbose", no_argument,       &verbose_flag, 1},
			{"brief",   no_argument,       &verbose_flag, 0},
			*/
			// The options dont set a flag.
			{"ip",             required_argument, 0, 'i'}, // ip address
			{"username",       required_argument, 0, 'u'}, // user name
			{"locale",         required_argument, 0, 'l'}, // GUI locale
			{"interface",      required_argument, 0, 'n'}, // network interface name
			{"download",       required_argument, 0, 'd'}, // configure auto download
			{"max_storage",    required_argument, 0, 's'}, // file sharing configuration
			{0, 0, 0, 0}
		};
		/* getopt_long stores the option index here. */
		int option_index = 0;

		c = getopt_long (argc, argv, "i:u:l:n:d:s:",
						long_options, &option_index);

		/* Detect the end of the options. */
		if (c == -1)
		break;

		switch (c)
		{
			case 0:
				// If this option set a flag, do nothing else now.
				if (long_options[option_index].flag != 0)
					break;

				printf ("option %s", long_options[option_index].name);

				if (optarg)
					printf (" with arg %s", optarg);

				printf ("\n");
				break;

			case 'i':
				// validate IP
				printf ("option -i with value `%s'\n", optarg);
				if(Qaullib_SetIP(optarg)!=1)
					printf ("ERROR: IP invalid\n");
				break;

			case 'u':
				if(Qaullib_StringNameProtect(protected_username, optarg, sizeof(protected_username)) > 0)
				{
					printf("save user name len %i: ", (int)strlen(protected_username));
					printf("%s  \n", protected_username);
					Qaullib_SetUsername(protected_username);
				}
				else
					printf("ERROR: user name invalid\n");
				break;

			case 'l':
				printf ("option -l with value `%s'\n", optarg);
				if(strlen(optarg) == 2)
				{
					Qaullib_SetLocale(optarg);
				}
				else
					printf("ERROR: locale to long\n");
				break;

			case 'n':
				printf ("option -n with value `%s'\n", optarg);
				if(validate_interface(optarg))
				{
					Qaullib_SetInterface(optarg);
					Qaullib_SetInterfaceManual(1);
				}
				else
					printf("ERROR: interface name invalid\n");
				break;

			case 'd':
				printf ("option -d with value `%s'\n", optarg);
				if(strncmp("1", optarg, 1) == 0)
					Qaullib_SetConfInt("files.autodownload", 1);
				else
					Qaullib_SetConfInt("files.autodownload", 0);
				break;

			case 's':
				printf ("option -s with value `%s'\n", optarg);
				if(max_storage_space = atoi(optarg))
				{
					Qaullib_SetConfInt("files.space.max", max_storage_space);
				}
				else
					printf("ERROR: max_storage is not a number\n");
				break;

			case '?':
				// getopt_long already printed an error message.
				break;

			default:
				abort ();
		}
	}

	// Print any remaining command line arguments (not options).
	if (optind < argc)
	{
		printf ("Unknown CLI options: ");
		while (optind < argc)
			printf ("%s ", argv[optind++]);
		putchar ('\n');
	}

}
