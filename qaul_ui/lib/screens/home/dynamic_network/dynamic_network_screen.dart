import 'dart:async' as async;
import 'dart:io';
import 'dart:math' as math;

import 'package:equatable/equatable.dart';
import 'package:flame/components.dart';
import 'package:flame/events.dart';
import 'package:flame/game.dart';
import 'package:flame_forge2d/flame_forge2d.dart';
import 'package:flutter/cupertino.dart' hide Draggable;
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart' hide Draggable;
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:intersperse/intersperse.dart';
import 'package:open_simplex_2/open_simplex_2.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:utils/utils.dart';

import '../../../providers/providers.dart';
import '../../../widgets/widgets.dart';

part 'models/network_node.dart';

part 'widgets/network_node_info_bottom_sheet.dart';

part 'widgets/network_type_filter.dart';

class DynamicNetworkScreen extends HookConsumerWidget {
  const DynamicNetworkScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final nodes = ref.watch(_filteredNodes);

    var gameEngine = _DynamicNetworkGameEngine(root: nodes);

    final currentTab = ref.watch(homeScreenControllerProvider);
    if (TabType.values[currentTab] == TabType.network) {
      gameEngine.resumeEngine();
    } else {
      gameEngine.pauseEngine();
    }

    return Scaffold(
      body: Stack(
        alignment: AlignmentDirectional.topEnd,
        children: [
          InteractiveViewer(child: GameWidget(game: gameEngine)),
          const _NetworkTypeFilterToolbar(),
        ],
      ),
    );
  }
}

class _DynamicNetworkGameEngine extends Forge2DGame
    with HasTappables, HasDraggables {
  _DynamicNetworkGameEngine({required this.root})
      : super(gravity: Vector2(0, 0));
  final NetworkNode root;

  @override
  Color backgroundColor() => Colors.transparent;

  @override
  Future<void> onLoad() async {
    await super.onLoad();
    final boundaries = _createBoundaries(this);
    boundaries.forEach(add);

    final worldCenter = screenToWorld(size * camera.zoom / 2);
    add(_NetworkNodeComponent(
      root,
      worldCenter,
      radius: 8,
      openBottomSheetOnTap: false,
    ));
  }
}

List<_Wall> _createBoundaries(Forge2DGame game) {
  final topLeft = Vector2.zero();
  final bottomRight = game.screenToWorld(game.camera.viewport.effectiveSize);
  final topRight = Vector2(bottomRight.x, topLeft.y);
  final bottomLeft = Vector2(topLeft.x, bottomRight.y);

  return [
    _Wall(topLeft, topRight),
    _Wall(topRight, bottomRight),
    _Wall(bottomRight, bottomLeft),
    _Wall(bottomLeft, topLeft),
  ];
}

class _Wall extends BodyComponent {
  final Vector2 start;
  final Vector2 end;

  _Wall(this.start, this.end) {
    // Set to true by default in BodyComponents
    debugMode = false;
  }

  @override
  Body createBody() {
    final shape = EdgeShape()..set(start, end);

    final fixtureDef = FixtureDef(shape)
      ..restitution = 0.0
      ..friction = 0.3;

    final bodyDef = BodyDef()
      ..userData = this // To be able to determine object in collision
      ..position = Vector2.zero()
      ..type = BodyType.static;

    return world.createBody(bodyDef)..createFixture(fixtureDef);
  }
}

class _NetworkNodeComponent extends BodyComponent
    with Tappable, Draggable, ContactCallbacks {
  _NetworkNodeComponent(
    this.node,
    this._position, {
    this.radius = 2,
    this.level = 0,
    this.initialDirection,
    this.ballParent,
    this.openBottomSheetOnTap = true,
  }) {
    // Node is painted manually on render()
    paint = Paint()..color = Colors.transparent;
  }

  final double radius;
  final Vector2 _position;

  final bool openBottomSheetOnTap;

  final NetworkNode node;
  final int level;
  final Vector2? initialDirection;

  final _NetworkNodeComponent? ballParent;

  final _noise = OpenSimplex2F(math.Random().nextInt(255));

  async.Timer? _timer;

  MouseJoint? mouseJoint;
  late Body groundBody;

  double get x => body.position.x;

  double get y => body.position.y;

  void addNoise(int t) {
    final scl = 8 * radius;
    final dx = scl * periodicFunction(t / 3 - offset(x, y), 0, x, y);
    final dy = scl * periodicFunction(t / 3 - offset(x, y), 123, x, y);

    var force = Vector2(dx, dy);
    if (initialDirection != null) {
      force.rotate(force.dot(initialDirection!));
    }
    body.applyForce(force);
  }

  double periodicFunction(double p, double seed, double x, double y) {
    return _noise.noise4Classic(
      seed + radius * math.cos(2 * math.pi * p),
      radius * math.sin(2 * math.pi * p),
      x,
      y,
    );
  }

  double offset(double x, double y) {
    return 0.015 *
        math.sqrt(math.pow(radius / 2 - x, 2) + math.pow(radius / 2 - y, 2));
  }

  void restartTimer() {
    if (_timer != null) return;
    _timer = async.Timer.periodic(
      const Duration(milliseconds: 100),
      (t) => addNoise(t.tick),
    );
  }

  // **************************
  // BodyComponent methods
  // **************************
  @override
  Future<void> onLoad() async {
    await super.onLoad();
    if (ballParent != null) restartTimer();

    body.setFixedRotation(true);

    groundBody = world.createBody(BodyDef());

    var i = 0;
    for (final child in node.children ?? {}) {
      var numberOfChildren = (node.children!.length);
      final angle = i * (2 * math.pi / numberOfChildren);
      i++;

      var newRadius = radius * .65 / numberOfChildren;

      var polar = Vector2(math.cos(angle), math.sin(angle));

      if (initialDirection != null) {
        polar.rotate(polar.dot(initialDirection!), center: _position);
      }
      final scaled = polar.clone()..scaleTo((radius + newRadius));

      var component = _NetworkNodeComponent(
        child,
        _position - scaled,
        radius: newRadius,
        level: level + 1,
        initialDirection: polar,
        ballParent: this,
      );

      gameRef.add(component);
    }
  }

  @override
  Body createBody() {
    // 1. Create a BodyDef
    final bodyDef = BodyDef()
      // To be able to determine object in collision
      ..userData = this
      ..angularDamping = 0.8
      ..position = _position
      ..type = BodyType.dynamic;

    // 2. Create a shape
    final shape = CircleShape();
    shape.radius = radius;

    // 3. Create a fixture
    final fixtureDef = FixtureDef(shape)
      ..restitution = 0.8
      ..density = 1.0
      ..friction = 0.4;

    // 4. Create the Body
    final body = world.createBody(bodyDef)..createFixture(fixtureDef);

    // 5. Create a Joint between child and its parent node
    if (ballParent != null) {
      final jointDef = DistanceJointDef();

      // The anchors imply the maximum distance (https://box2d.org/documentation/md__d_1__git_hub_box2d_docs_dynamics.html#autotoc_md85)
      jointDef.initialize(
        body,
        ballParent!.body,
        body.position,
        ballParent!.body.position,
      );

      jointDef.collideConnected = true;
      jointDef.dampingRatio = .2;
      jointDef.frequencyHz = 0;

      world.createJoint(DistanceJoint(jointDef));
    } else {
      final center = gameRef.screenToWorld(gameRef.size * camera.zoom / 2);
      final fixedPoint =
          world.createBody(BodyDef(type: BodyType.static, position: center));

      final jointDef = RopeJointDef()
        ..maxLength = 20.0
        ..collideConnected = false
        ..bodyA = fixedPoint
        ..bodyB = body;

      world.createJoint(RopeJoint(jointDef));
    }

    return body;
  }

  @override
  void render(Canvas canvas) {
    super.render(canvas);
    var circle = Paint()..color = colorGenerationStrategy(node.user.idBase58);
    canvas.drawCircle(Offset.zero, radius, circle);

    const factor = 10;
    final fontSize = radius * factor;

    // Remove too small to see initials
    if (fontSize < factor) return;

    if (kDebugMode) {
      canvas.drawCircle(Offset.zero, .2, Paint()..color = Colors.red);
    }

    final proportionalFontSize = gameRef.screenToWorld(Vector2(fontSize, 0)).x;

    final style = TextStyle(
      fontSize: proportionalFontSize,
      color: Colors.white,
      fontWeight: FontWeight.w700,
      // y = 0.0625x+0.7
      height: 0.7 + 0.0625 * proportionalFontSize,
    );
    final tp = TextPainter(
      text: TextSpan(text: initials(node.user.name), style: style),
      textAlign: TextAlign.center,
      textDirection: TextDirection.ltr,
    );
    tp.layout();
    tp.paint(canvas, Offset.zero - tp.size.center(Offset.zero));
  }

  // **************************
  // Tappable methods
  // **************************
  @override
  bool onTapDown(_) {
    if (openBottomSheetOnTap && gameRef.buildContext != null) {
      Scaffold.of(gameRef.buildContext!).showBottomSheet(
        (context) => _NetworkNodeInfoBottomSheet(node: node),
        backgroundColor: Colors.transparent,
      );
    }
    return false;
  }

  // **************************
  // Draggable methods
  // **************************
  @override
  bool onDragStart(DragStartInfo info) {
    if (ballParent != null) return true;

    _timer?.cancel();
    return false;
  }

  @override
  bool onDragUpdate(DragUpdateInfo info) {
    if (ballParent != null) return true;

    final mouseJointDef = MouseJointDef()
      ..maxForce = 3000 * body.mass * 10
      ..dampingRatio = 0.1
      ..frequencyHz = 5
      ..target.setFrom(body.position)
      ..collideConnected = false
      ..bodyA = groundBody
      ..bodyB = body;

    if (mouseJoint == null) {
      mouseJoint = MouseJoint(mouseJointDef);
      world.createJoint(mouseJoint!);
    }
    mouseJoint?.setTarget(info.eventPosition.game);
    return false;
  }

  @override
  bool onDragEnd(DragEndInfo info) {
    if (ballParent != null || mouseJoint == null) return true;

    world.destroyJoint(mouseJoint!);
    mouseJoint = null;
    restartTimer();
    return false;
  }

  @override
  bool onDragCancel() {
    if (ballParent != null) return true;

    restartTimer();
    return false;
  }

  // **************************
  // ContactCallbacks Methods
  // **************************
  @override
  void beginContact(Object other, Contact contact) {
    if (other is! _NetworkNodeComponent) return;

    var nodesInSameLevel = level == other.level;
    var aHitsParentSibling = level == other.level + 1 && ballParent != other;
    if (nodesInSameLevel || aHitsParentSibling) {
      final dist =
          contact.bodyA.position.distanceToSquared(contact.bodyB.position);
      final overlap = 0.5 * (dist - radius - other.radius);

      final x = body.position.x -
          (overlap * (body.position.x - other.body.position.x) / dist);
      final y = body.position.y -
          (overlap * (body.position.y - other.body.position.y) / dist);

      contact.bodyA.applyForce(Vector2(x, y));
      contact.bodyB.applyForce(Vector2(-x, -y));
    }
  }
}
