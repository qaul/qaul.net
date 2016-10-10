/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <stdio.h> // defines FILENAME_MAX
#include <stdlib.h>
#include <getopt.h>

#include "../../libqaul/qaullib_private.h"
#include "cli_options.h"

void qaul_cli_options(int argc, char *argv[])
{
	int c;
	char protected_username[MAX_USER_LEN +1];

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
			{"ip",             required_argument, 0, 'i'},
			{"username",       required_argument, 0, 'u'},
			{"interface",      required_argument, 0, 'n'},
			{"auto_download",  no_argument,       0, 'a'}, // file sharing configuration
			{"manual_download",no_argument,       0, 'm'}, // file sharing configuration
			{"max_storage",    required_argument, 0, 's'}, // file sharing configuration
			{0, 0, 0, 0}
		};
		/* getopt_long stores the option index here. */
		int option_index = 0;

		c = getopt_long (argc, argv, "am:d:f:",
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
				puts ("option -a\n");
				break;

			case 'u':
				if(Qaullib_StringNameProtect(protected_username, optarg, sizeof(protected_username)) > 0)
				{
					printf("save user name len %i: ", (int)strlen(protected_username));
					printf("%s  \n", protected_username);
					Qaullib_SetUsername(protected_username);
				}
				break;

			case 'n':
				printf ("option -n with value `%s'\n", optarg);
				break;

			case 'a':
				printf ("option -a with value `%s'\n", optarg);
				break;

			case 'm':
				printf ("option -m with value `%s'\n", optarg);
				break;

			case 's':
			  printf ("option -s with value `%s'\n", optarg);
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
