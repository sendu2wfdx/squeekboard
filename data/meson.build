gnome = import('gnome')

#workaround due to https://github.com/mesonbuild/meson/issues/1687
copy_schema = custom_target('copy-gschema-to-builddir',
  input: 'sm.puri.Squeekboard.gschema.xml',
  output: 'sm.puri.Squeekboard.gschema.xml',
  command: ['cp', '@INPUT@', '@OUTPUT@']
)
schemas = ['sm.puri.Squeekboard.gschema.xml']
compile_schemas = custom_target('glib-compile-schemas',
  build_by_default: true,
  output: 'gschemas.compiled',
  install: false,
  command: [find_program('glib-compile-schemas'),
    meson.current_build_dir()
  ],
  depends: [copy_schema]
)
install_data(
  schemas,
  install_dir: schemasdir
)

compile_schemas = find_program('glib-compile-schemas', required: false)
if compile_schemas.found()
  test('Validate schema file', compile_schemas,
       args: ['--strict', '--dry-run', meson.current_source_dir()])
endif

install_data(
  schemas,
  install_dir: 'share/glib-2.0/schemas'
)

squeekboard_resources = gnome.compile_resources(
   'squeekboard-resources',
   'squeekboard.gresources.xml',

   c_name: 'squeekboard',
)

desktopconf = configuration_data()
desktopconf.set('bindir', bindir)

desktop_file = 'sm.puri.Squeekboard.desktop'

i18n.merge_file(
    input: configure_file(
      input: desktop_file + '.in.in',
      output: desktop_file + '.in',
      configuration: desktopconf
    ),
    output: desktop_file,
    po_dir: '../po',
    install: true,
    install_dir: desktopdir,
    type: 'desktop'
)

configure_file(
  input: 'sm.puri.Squeekboard.service.in',
  output: 'mobi.phosh.OSK.service',
  configuration: desktopconf,
  install: true,
  install_dir: systemd_user_unit_dir,
)
