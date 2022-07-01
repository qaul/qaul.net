#!/usr/bin/python3
# This script provides a quick fix to the issue documented here: https://github.com/flutter/flutter/issues/91668

def inplace_change(filename, old_string, new_string):
    # Safely read the input filename using 'with'
    with open(filename) as f:
        s = f.read()
        if old_string not in s:
            print('"{old_string}"\nnot found in {filename}.'.format(**locals()))
            return

    # Safely write the changed content, if found in the file
    with open(filename, 'w') as f:
        print('Changing:\n"{old_string}"\nto:\n"{new_string}"\nin {filename}'.format(**locals()))
        s = s.replace(old_string, new_string)
        f.write(s)

filename = 'ios/.symlinks/plugins/integration_test/ios/Classes/IntegrationTestPlugin.m'

old_string = """+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar> *)registrar {
  // No initialization happens here because of the way XCTest loads the testing
  // bundles.  Setup on static variables can be disregarded when a new static
  // instance of IntegrationTestPlugin is allocated when the bundle is reloaded.
  // See also: https://github.com/flutter/plugins/pull/2465
}"""

new_string = """+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar> *)registrar {
  [[IntegrationTestPlugin instance] setupChannels:registrar.messenger];
}"""

inplace_change(filename=filename, old_string=old_string, new_string=new_string)
