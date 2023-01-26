# Adding a new translation
Whenever you're adding a new translation, you can follow these general guidelines:

1. Copy the english translation file (which is the template), and replace `en` with the [Language Tag](https://en.wikipedia.org/wiki/IETF_language_tag) being added:
`cp qaul_ui/lib/l10n/app_en.arb qaul_ui/lib/l10n/app_<YOUR TAG HERE>.arb`
2. Remove *all* translation blocks starting with `@` in their name:
```shell
# EXAMPLE...
diff --git a.arb b.arb
index 6266b81b..f8a05761 100644
--- a.arb
+++ b.arb
@@ -1,9 +1,6 @@
 {
   "languageName": "اللغة العربية",
   "createAccountHeading": "اختر اسم مستخدم",
-  "@createAccountHeading": {
-    "description": "CTA rendered on top of text field"
-  },
```
> The reason for it is that these are not translations, but rather [resource attribute](https://github.com/google/app-resource-bundle/wiki/ApplicationResourceBundleSpecification#resource-attributes), 
> which only needs to be defined in the template language - in qaul's case, `app_en.arb`
3. Translate the values for each key-value pair, ignoring any symbols enclosed in curly brackets - `{}`:
```shell
# EXAMPLE...
diff --git a.arb b.arb
index 342348a9..6baadda5 100644
--- a.arb
+++ b.arb
@@ -104,7 +104,7 @@
     # Don't do this ❌
-    "groupMemberEvent": "\"{nom d'utilisateur}\"a {événement} le groupe",
     # That's the correct form 👍
+    "groupMemberEvent": "\"{username}\" a {event} le groupe",
```
> The elements within curly brackets are [placeholders](https://github.com/google/app-resource-bundle/wiki/ApplicationResourceBundleSpecification#placeholder-in-resource),
> and that's how we pass in values to the text that will only be fulfilled while the app is running.
