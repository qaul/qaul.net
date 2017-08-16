



/** Holds data about a node */
typedef struct qluser_node_t {
    union olsr_ip_addr  ip;
    struct qluser_t     **identities;
};

/** Holds data about a user identity */
typedef struct qluser_t {
    const char *fp;
    const char *pubkey;
    char *name;
    qluser_node_t *node;
};

