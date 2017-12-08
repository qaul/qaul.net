/* qaul.net - libqaul
 *
 * libqaul implements and uses many data structures to move data around
 * it's core components. While all external calls use a common msgpack
 * interface, internally only simple structures are used.
 *
 * These structures, types and enums are declared here. Each file in the
 * library can then include this file and have access to all datastructures
 * that are available in the library. This also avoids duplicating
 * functionality between two structures in two different modules.
 *
 * If this file get's too long it can be split up into smaller modules
 *
 * ----------------------------------------------------------------------------
 *
 * This program and the accompanying materials
 * are made available under the terms of the GNU Lesser General Public License
 * (LGPL) version 3 which accompanies this distribution, and is available at
 * http://www.gnu.org/licenses/lgpl-3.html
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * Lesser General Public License for more details.
 *
 */


#ifndef QAUL_QLFORMAT_H
#define QAUL_QLFORMAT_H

/**
 * A
 */
typedef enum ql_crydata_t {
    FINGERPRINT,
    PUBKEY,
};

typedef enum ql_crykey_t {
    RSA, AES256
};

typedef struct ql_pubkey {
    enum ql_crykey_t type;
};

typedef struct ql_seckey {
    enum ql_crykey_t type;
};

typedef struct ql_keypair {
    enum ql_crykey_t type;
    struct ql_pubkey *pub;
    struct ql_seckey *sec;
};


/**
 * A qaul user struct
 *
 * Contains basic data that needs to be known
 */
typedef struct ql_user {
    const char *username;
    const char *fingerprint;
};


#endif //QAUL_QLFORMAT_H
