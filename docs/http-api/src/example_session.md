# So You Want To Use the HTTP API
This is a basic tutorial covering how to get started with the qaul.net HTTP API

The API is built on the principles of [JSON:API](https://jsonapi.org/format/) so while not
strictly nessicary it is highly encouraged you read the spec before starting.

Before we can do anything we need a user and that means creating a new `secret`
```
curl -i  \
    -H "Content-Type: application/vnd.api+json" \
    -d '{
        "data": {
            "type": "secret",
            "attributes": {
                "value": "MyFairSecret"
            }
        }
    }' \
    "http://0.0.0.0:9090/api/secrets"
```
which will return something like
```
{
  "data": {
    "id": "2a6463711f0e149a0262a34fc79b9e16",
    "type": "secret",
    "relationships": {
      "user": {
        "data": {
          "id": "2a6463711f0e149a0262a34fc79b9e16",
          "type": "user"
        }
      }
    },
    "links": {
      "self": "/api/secrets/2a6463711f0e149a0262a34fc79b9e16"
    }
  }
}
```
The `id` field of `data` is our user id, `2a6463711f0e149a0262a34fc79b9e16`.

Now we should get a `grant` for our user, which will be used to authenticate our
application to the API.
```
curl -i  \
    -H "Content-Type: application/vnd.api+json" \
    -d '{
        "data": {
            "type": "grant",
            "attributes": { 
                "secret": "MyFairSecret"
            },
            "relationships": {
                "user": {
                    "data": { 
                         "type": "user",
                         "id": "2a6463711f0e149a0262a34fc79b9e16"
                    }
                }
            }
        }
    }' \
    "http://0.0.0.0:9090/api/grants"
```
Notice that we provide the id of the user we're trying to get a grant for as a relationship.
This will return something like:
```
{
  "data": {
    "id": "hZ4g3IGKsGlZarvJr4EYGSaMtBGCqbTv13g47KuKQawGG9p2PHPOuuG6DDv05AGjb5090a3wW6YE_-iTHxgmkpeLeYv7DOILj9_mNd6kcGmrV3X-UiBxIIXIWymV-yLhbnY5J9B-FEZsOPJNpzd9rNAe8SKV_5v2Y1jPpqERYBJcLAwd2NQwz531t1Oq7i8j3H2weDNOZijc6HN-B7_DTUSudKIPbUx7g1UnLlq4J22ezwFe8KHmA12ANDfCgzT2GAMjGgB-B0QvicvsCP-6TemCHSF0fctFoGuYRX25DALon0pf8N0K8kGMZtV-WongJi0ns2gAfbWl9JjAx1ssfw==",
    "type": "grant",
    "relationships": {
      "user": {
        "data": {
          "id": "2a6463711f0e149a0262a34fc79b9e16",
          "type": "user"
        }
      }
    },
    "links": {
      "self": "/api/grants/hZ4g3IGKsGlZarvJr4EYGSaMtBGCqbTv13g47KuKQawGG9p2PHPOuuG6DDv05AGjb5090a3wW6YE_-iTHxgmkpeLeYv7DOILj9_mNd6kcGmrV3X-UiBxIIXIWymV-yLhbnY5J9B-FEZsOPJNpzd9rNAe8SKV_5v2Y1jPpqERYBJcLAwd2NQwz531t1Oq7i8j3H2weDNOZijc6HN-B7_DTUSudKIPbUx7g1UnLlq4J22ezwFe8KHmA12ANDfCgzT2GAMjGgB-B0QvicvsCP-6TemCHSF0fctFoGuYRX25DALon0pf8N0K8kGMZtV-WongJi0ns2gAfbWl9JjAx1ssfw=="
    }
  }
}
```
The `id` field of `data` here is our grant. We will pass this in the future as a bearer auth token.

Let's give our user a name and fill out their profile a bit:
```
curl -i  \
    -X PATCH \
    -H "Content-Type: application/vnd.api+json" \
    -H "Authorization: Bearer hZ4g3IGKsGlZarvJr4EYGSaMtBGCqbTv13g47KuKQawGG9p2PHPOuuG6DDv05AGjb5090a3wW6YE_-iTHxgmkpeLeYv7DOILj9_mNd6kcGmrV3X-UiBxIIXIWymV-yLhbnY5J9B-FEZsOPJNpzd9rNAe8SKV_5v2Y1jPpqERYBJcLAwd2NQwz531t1Oq7i8j3H2weDNOZijc6HN-B7_DTUSudKIPbUx7g1UnLlq4J22ezwFe8KHmA12ANDfCgzT2GAMjGgB-B0QvicvsCP-6TemCHSF0fctFoGuYRX25DALon0pf8N0K8kGMZtV-WongJi0ns2gAfbWl9JjAx1ssfw==" \
    -d '{
        "data": {
            "id": "2a6463711f0e149a0262a34fc79b9e16",
            "type": "user",
            "attributes": { 
                "display_name": "tester",
                "real_name": "Tester McTesterson III",
                "bio": {
                    "gender": "I would really rather not",
                    "shirt color": "red"
                }
            }
        }
    }' \
    "http://0.0.0.0:9090/api/users/2a6463711f0e149a0262a34fc79b9e16"
```
which returns the updated user record
```
{
  "data": {
    "id": "2a6463711f0e149a0262a34fc79b9e16",
    "type": "user",
    "attributes": {
      "bio": {
        "gender": "I would really rather not",
        "shirt color": "red"
      },
      "display_name": "tester",
      "real_name": "Tester McTesterson III",
      "services": []
    },
    "relationships": {
      "secret": {
        "data": {
          "id": "2a6463711f0e149a0262a34fc79b9e16",
          "type": "secret"
        }
      }
    },
    "links": {
      "self": "/api/users/2a6463711f0e149a0262a34fc79b9e16"
    }
  }
}
```

Let's say our user has met a new friend with id `f7335eb601789ddbe0cd76f6915eb3fe`. They perform
a look up to find the user's profile:
```
curl -i  \
    -H "Content-Type: application/vnd.api+json" \
    -H "Authorization: Bearer hZ4g3IGKsGlZarvJr4EYGSaMtBGCqbTv13g47KuKQawGG9p2PHPOuuG6DDv05AGjb5090a3wW6YE_-iTHxgmkpeLeYv7DOILj9_mNd6kcGmrV3X-UiBxIIXIWymV-yLhbnY5J9B-FEZsOPJNpzd9rNAe8SKV_5v2Y1jPpqERYBJcLAwd2NQwz531t1Oq7i8j3H2weDNOZijc6HN-B7_DTUSudKIPbUx7g1UnLlq4J22ezwFe8KHmA12ANDfCgzT2GAMjGgB-B0QvicvsCP-6TemCHSF0fctFoGuYRX25DALon0pf8N0K8kGMZtV-WongJi0ns2gAfbWl9JjAx1ssfw==" \
    "http://0.0.0.0:9090/api/users/f7335eb601789ddbe0cd76f6915eb3fe"
```
which returns
```
{
  "data": {
    "id": "f7335eb601789ddbe0cd76f6915eb3fe",
    "type": "user",
    "attributes": {
      "bio": {
        "gender": "yes please"
      },
      "display_name": "friend",
      "real_name": "Friendly, First of Their Name",
      "services": []
    },
    "relationships": {
      "secret": {
        "data": {
          "id": "f7335eb601789ddbe0cd76f6915eb3fe",
          "type": "secret"
        }
      }
    },
    "links": {
      "self": "/api/users/f7335eb601789ddbe0cd76f6915eb3fe"
    }
  }
}
```

The user is done with this session so they decide to log out:
```
curl -i  \
    -X DELETE \
    -H "Content-Type: application/vnd.api+json" \
    -H "Authorization: Bearer hZ4g3IGKsGlZarvJr4EYGSaMtBGCqbTv13g47KuKQawGG9p2PHPOuuG6DDv05AGjb5090a3wW6YE_-iTHxgmkpeLeYv7DOILj9_mNd6kcGmrV3X-UiBxIIXIWymV-yLhbnY5J9B-FEZsOPJNpzd9rNAe8SKV_5v2Y1jPpqERYBJcLAwd2NQwz531t1Oq7i8j3H2weDNOZijc6HN-B7_DTUSudKIPbUx7g1UnLlq4J22ezwFe8KHmA12ANDfCgzT2GAMjGgB-B0QvicvsCP-6TemCHSF0fctFoGuYRX25DALon0pf8N0K8kGMZtV-WongJi0ns2gAfbWl9JjAx1ssfw==" \
    "http://0.0.0.0:9090/api/grants/hZ4g3IGKsGlZarvJr4EYGSaMtBGCqbTv13g47KuKQawGG9p2PHPOuuG6DDv05AGjb5090a3wW6YE_-iTHxgmkpeLeYv7DOILj9_mNd6kcGmrV3X-UiBxIIXIWymV-yLhbnY5J9B-FEZsOPJNpzd9rNAe8SKV_5v2Y1jPpqERYBJcLAwd2NQwz531t1Oq7i8j3H2weDNOZijc6HN-B7_DTUSudKIPbUx7g1UnLlq4J22ezwFe8KHmA12ANDfCgzT2GAMjGgB-B0QvicvsCP-6TemCHSF0fctFoGuYRX25DALon0pf8N0K8kGMZtV-WongJi0ns2gAfbWl9JjAx1ssfw=="
```
