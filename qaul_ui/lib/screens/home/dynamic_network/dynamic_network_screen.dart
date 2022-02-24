import 'dart:async' as async;
import 'dart:math' as math;
import 'dart:math';
import 'dart:typed_data';

import 'package:equatable/equatable.dart';
import 'package:flame/components.dart';
import 'package:flame/game.dart';
import 'package:flame_forge2d/body_component.dart';
import 'package:flame_forge2d/contact_callbacks.dart';
import 'package:flame_forge2d/forge2d_game.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:forge2d/forge2d.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:open_simplex_2/open_simplex_2.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:utils/utils.dart';

import '../../../widgets/widgets.dart';

part 'models/network_node.dart';
part 'widgets/network_type_filter.dart';

class DynamicNetworkScreen extends HookConsumerWidget {
  const DynamicNetworkScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final nodes = ref.watch(_filteredNodes);

    return Scaffold(
      body: Stack(
        alignment: AlignmentDirectional.topEnd,
        children: [
          InteractiveViewer(
            child: GameWidget(
              game: _DynamicNetworkGameEngine(root: nodes),
            ),
          ),
          const _NetworkTypeFilterToolbar(),
        ],
      ),
    );
  }
}

class _DynamicNetworkGameEngine extends Forge2DGame with HasTappables {
  _DynamicNetworkGameEngine({required this.root}) : super(gravity: Vector2(0, 0));
  final NetworkNode root;

  @override
  Color backgroundColor() => Colors.transparent;

  @override
  Future<void> onLoad() async {
    await super.onLoad();
    final boundaries = _createBoundaries(this);
    boundaries.forEach(add);
    addContactCallback(_NudgeSiblingsAndNephewsCallback());

    final worldCenter = screenToWorld(size * camera.zoom / 2);
    add(_NetworkNodeComponent(root, worldCenter, radius: 8));
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

class _NudgeSiblingsAndNephewsCallback
    extends ContactCallback<_NetworkNodeComponent, _NetworkNodeComponent> {
  @override
  void begin(_NetworkNodeComponent a, _NetworkNodeComponent b, Contact contact) {
    var nodesInSameLevel = a.level == b.level;
    var aHitsParentSibling = a.level == b.level + 1 && a.ballParent != b;
    if (nodesInSameLevel || aHitsParentSibling) {
      final dist = contact.bodyA.position.distanceToSquared(contact.bodyB.position);
      final overlap = 0.5 * (dist - a.radius - b.radius);

      final x = a.body.position.x - (overlap * (a.body.position.x - b.body.position.x) / dist);
      final y = a.body.position.y - (overlap * (a.body.position.y - b.body.position.y) / dist);

      contact.bodyA.applyForce(Vector2(x, y));
      contact.bodyB.applyForce(Vector2(-x, -y));
    }
  }

  @override
  void end(_NetworkNodeComponent a, _NetworkNodeComponent b, Contact contact) {}
}

class _NetworkNodeComponent extends BodyComponent with Tappable {
  _NetworkNodeComponent(
    this.node,
    this._position, {
    this.radius = 2,
    this.level = 0,
    this.initialDirection,
    this.ballParent,
  }) {
    // Painted manually on render()
    paint = Paint()..color = Colors.transparent;
  }

  final double radius;
  final Vector2 _position;

  final NetworkNode node;
  final int level;
  final Vector2? initialDirection;

  final _NetworkNodeComponent? ballParent;

  final _noise = OpenSimplex2S(10);

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
    return 0.015 * math.sqrt(math.pow(radius / 2 - x, 2) + math.pow(radius / 2 - y, 2));
  }

  @override
  Future<void> onLoad() async {
    await super.onLoad();

    // Add noise callback
    async.Timer.periodic(
      const Duration(milliseconds: 100),
      (t) => addNoise(t.tick),
    );

    var i = 0;
    for (final child in node.children ?? {}) {
      final angle = i * (2 * math.pi / (node.children!.length));
      i++;

      // TODO must be a factor of the number of children
      var newRadius = radius * .45;

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
      jointDef.frequencyHz = 1.0;

      world.createJoint(jointDef);
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

  @override
  bool onTapDown(_) {
    if (gameRef.buildContext != null) {
      Scaffold.of(gameRef.buildContext!).showBottomSheet(
        (context) {
          return const _NetworkNodeInfoBottomSheet();
        },
        backgroundColor: Colors.transparent,
      );
    }
    return false;
  }
}

class _NetworkNodeInfoBottomSheet extends StatelessWidget {
  const _NetworkNodeInfoBottomSheet({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l18ns = AppLocalizations.of(context)!;
    final theme = Theme.of(context).textTheme;

    const start = Alignment.centerLeft;
    const end = Alignment.centerRight;

    return GestureDetector(
      child: Container(
        margin: const EdgeInsets.symmetric(horizontal: 20.0),
        padding: const EdgeInsets.fromLTRB(20.0, 16.0, 10.0, 16.0),
        decoration: BoxDecoration(
          color: Theme.of(context).appBarTheme.backgroundColor,
          borderRadius: const BorderRadius.vertical(
            top: Radius.circular(20.0),
          ),
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.end,
          mainAxisAlignment: MainAxisAlignment.start,
          children: <Widget>[
            Align(
              alignment: Alignment.topLeft,
              child: GestureDetector(
                onTap: () => Navigator.pop(context),
                child: const Icon(Icons.close_rounded),
              ),
            ),
            ListTile(
              leading: UserAvatar.small(),
              visualDensity: VisualDensity.adaptivePlatformDensity,
              contentPadding: EdgeInsets.zero,
              title: Padding(
                padding: const EdgeInsets.only(bottom: 4.0),
                child: Text('Name name', style: theme.headline6),
              ),
              subtitle: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'ID: --------------------------------------',
                    style: theme.caption!.copyWith(fontSize: 10),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 12),
            Table(
              defaultVerticalAlignment: TableCellVerticalAlignment.middle,
              columnWidths: const {
                0: FlexColumnWidth(.05),
                1: FlexColumnWidth(.3),
                2: FlexColumnWidth(.3),
                3: FlexColumnWidth(.3),
              },
              children: [
                TableRow(
                  children: [
                    const TableCell(child: SizedBox.shrink()),
                    TableCell(child: Text(l18ns.ping)),
                    TableCell(child: Text(l18ns.hopCount)),
                    TableCell(child: Text(l18ns.via)),
                  ],
                ),
                TableRow(children: [
                  const TableCell(
                    child: Align(
                      alignment: start,
                      child: Icon(Icons.bluetooth),
                    ),
                  ),
                  TableCell(
                    child: Container(
                      alignment: end,
                      padding: const EdgeInsets.only(right: 12),
                      child: const Text('200 ms'),
                    ),
                  ),
                  const TableCell(child: Text('HC')),
                  const TableCell(child: Text('Via')),
                ]),
                TableRow(children: [
                  const TableCell(
                    child: Align(
                      alignment: start,
                      child: Icon(Icons.wifi),
                    ),
                  ),
                  TableCell(
                    child: Container(
                      alignment: end,
                      padding: const EdgeInsets.only(right: 12),
                      child: const Text('200 ms'),
                    ),
                  ),
                  const TableCell(child: Text('HC')),
                  const TableCell(child: Text('Via')),
                ]),
                TableRow(children: [
                  const TableCell(
                    child: Align(
                      alignment: start,
                      child: Icon(CupertinoIcons.globe),
                    ),
                  ),
                  TableCell(
                    child: Container(
                      alignment: end,
                      padding: const EdgeInsets.only(right: 12),
                      child: const Text('200 ms'),
                    ),
                  ),
                  const TableCell(child: Text('HC')),
                  const TableCell(child: Text('Via')),
                ]),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
