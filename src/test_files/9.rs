fn hehe()
{
  let dep_filter = move | _p : WorkspacePackageRef< '_ >, d : DependencyRef< '_ > |
  {
    (
      args.dependency_categories.contains( &DependencyCategory::Primary ) && d.kind() == DependencyKind::Normal
      || args.dependency_categories.contains( &DependencyCategory::Primary ) && d.kind() == DependencyKind::Normal
    )
    && (
        args.dependency_sources.contains( &DependencySource::Remote ) && d.crate_dir().is_none()
        || args.dependency_categories.contains( &DependencyCategory::Primary ) && d.kind() == DependencyKind::Normal
      )
  };
}
