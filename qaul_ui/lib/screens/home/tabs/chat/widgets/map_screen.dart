import 'package:flutter/material.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';
import 'package:geolocator/geolocator.dart';

class MapScreen extends StatefulWidget {
  const MapScreen({
    Key? key,
    required this.onLocationSelected,
  }) : super(key: key);

  final Function(LatLng) onLocationSelected;

  @override
  _MapScreenState createState() => _MapScreenState();
}

class _MapScreenState extends State<MapScreen> {
  late GoogleMapController mapController;
  LatLng? _currentPosition;
  LatLng? _selectedPosition;

  @override
  void initState() {
    super.initState();
    _checkAndRequestLocationPermission();
  }

  Future<void> _checkAndRequestLocationPermission() async {
    bool serviceEnabled;
    LocationPermission permission;

    // Check if location services are enabled
    serviceEnabled = await Geolocator.isLocationServiceEnabled();
    if (!serviceEnabled) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Location services are disabled.')),
      );
      return;
    }

    // Check and request location permissions
    permission = await Geolocator.checkPermission();
    if (permission == LocationPermission.denied) {
      permission = await Geolocator.requestPermission();
      if (permission == LocationPermission.denied) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Location permissions are denied.')),
        );
        return;
      }
    }

    if (permission == LocationPermission.deniedForever) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text(
              'Location permissions are permanently denied. Please enable them in settings.'),
        ),
      );
      return;
    }

    // If permissions are granted, update the current location
    _updateCurrentLocation();
  }

  void _onMapCreated(GoogleMapController controller) {
    mapController = controller;
  }

  Future<void> _updateCurrentLocation() async {
    try {
      final position = await Geolocator.getCurrentPosition(
          desiredAccuracy: LocationAccuracy.high);
      setState(() {
        _currentPosition = LatLng(position.latitude, position.longitude);
      });
      mapController.animateCamera(
        CameraUpdate.newLatLng(_currentPosition!),
      );
    } catch (e) {
      debugPrint('Error getting location: $e');
    }
  }

  void _onMarkerDragged(LatLng position) {
    setState(() {
      _currentPosition = position;
    });
    widget.onLocationSelected(position);
  }

  void _onMapTap(LatLng position) {
    setState(() {
      _selectedPosition = position;
    });
  }

  void _onConfirmLocation() {
    if (_selectedPosition != null) {
      final roundedLat = _selectedPosition!.latitude.toStringAsFixed(2);
      final roundedLng = _selectedPosition!.longitude.toStringAsFixed(2);
      widget.onLocationSelected(LatLng(
        double.parse(roundedLat),
        double.parse(roundedLng),
      ));
      Navigator.pop(context);
    } else if (_currentPosition != null) {
      final roundedLat = _currentPosition!.latitude.toStringAsFixed(2);
      final roundedLng = _currentPosition!.longitude.toStringAsFixed(2);
      widget.onLocationSelected(LatLng(
        double.parse(roundedLat),
        double.parse(roundedLng),
      ));
      Navigator.pop(context);
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please select a location.')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Select Location'),
        actions: [
          IconButton(
            icon: const Icon(Icons.check),
            onPressed: _onConfirmLocation,
          ),
          IconButton(
            icon: const Icon(Icons.my_location),
            onPressed: _updateCurrentLocation,
          ),
        ],
      ),
      body: _currentPosition == null
          ? const Center(child: CircularProgressIndicator())
          : GoogleMap(
              onMapCreated: _onMapCreated,
              initialCameraPosition: CameraPosition(
                target: _currentPosition!,
                zoom: 12.0,
              ),
              onTap: _onMapTap,
              markers: _selectedPosition != null
                  ? {
                      Marker(
                        markerId: const MarkerId('selected-location'),
                        position: _selectedPosition!,
                        draggable: true,
                        onDragEnd: _onMarkerDragged,
                      ),
                    }
                  : {
                      Marker(
                        markerId: const MarkerId('selectedLocation'),
                        position: _currentPosition!,
                        draggable: true,
                        onDragEnd: _onMarkerDragged,
                      ),
                    },
            ),
    );
  }
}
