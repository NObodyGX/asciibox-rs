scalable_dir = join_paths('hicolor', 'scalable', 'apps')
install_data(
  join_paths(scalable_dir, f'@app_name@.svg'),
  install_dir: join_paths(get_option('datadir'), 'icons', scalable_dir)
)

symbolic_dir = join_paths('hicolor', 'symbolic', 'apps')
install_data(
  join_paths(symbolic_dir, f'@app_name@-symbolic.svg'),
  install_dir: join_paths(get_option('datadir'), 'icons', symbolic_dir)
)
