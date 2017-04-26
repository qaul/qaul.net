/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/// number of the configuration step
int qaulConfigureCounter;

/**
 * @val 1 a network interface has been found
 * @val 0 no network interface has been found
 */
int network_interface_found;


/**
 * qaul.net configuration loop
 *
 * @retval 1 configuration in progress
 * @retval 0 configuration finished
 */
int qaul_configure(void);

/**
 * start timers
 *
 * implement this function in main.c
 */
void qaul_startTimers(void);
