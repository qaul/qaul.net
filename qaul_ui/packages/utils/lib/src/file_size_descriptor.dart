extension FileSizeDescriptor on int {
  int get kb => this * 1000;

  int get mb => kb * 1000;

  int get gb => mb * 1000;
}
