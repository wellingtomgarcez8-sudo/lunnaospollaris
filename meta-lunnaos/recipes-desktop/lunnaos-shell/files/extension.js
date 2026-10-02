import St from 'gi://St';
import Clutter from 'gi://Clutter';
import Shell from 'gi://Shell';
import * as Main from 'resource:///org/gnome/shell/ui/main.js';

let dock;

function makeDock() {
  dock = new St.BoxLayout({style_class: 'lunnaos-dock', x_align: Clutter.ActorAlign.CENTER});
  const appSystem = Shell.AppSystem.get_default();
  const favorites = appSystem.get_favorite_map();
  Object.values(favorites).slice(0, 8).forEach(app => {
    const button = new St.Button({style_class: 'lunnaos-dock-button', reactive: true});
    button.set_child(app.create_icon_texture(36));
    button.connect('clicked', () => app.activate());
    dock.add_child(button);
  });
  Main.layoutManager.addChrome(dock, {trackFullscreen: true});
  Main.layoutManager.connectObject('monitors-changed', () => reposition(), dock);
  reposition();
}

function reposition() {
  if (!dock) return;
  const monitor = Main.layoutManager.primaryMonitor;
  dock.set_position(Math.floor(monitor.x + (monitor.width - dock.width) / 2), monitor.y + monitor.height - 82);
}

export default class LunnaOSShell {
  enable() { Main.panel.add_style_class_name('lunnaos-panel'); makeDock(); }
  disable() { Main.panel.remove_style_class_name('lunnaos-panel'); dock?.destroy(); dock = null; }
}