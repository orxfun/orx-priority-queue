/// Trait representing (node, key) pairs stored on priority queues.
pub trait NodeKeyRef<'a, N, K>
where
    N: 'a,
    K: 'a,
{
    /// Returns a reference to the node.
    fn node(&self) -> &'a N;
    /// Returns a reference to the key/priority of the node.
    fn key(&self) -> &'a K;
}

impl<'a, N, K> NodeKeyRef<'a, N, K> for &'a (N, K)
where
    N: 'a,
    K: 'a,
{
    fn node(&self) -> &'a N {
        &self.0
    }
    fn key(&self) -> &'a K {
        &self.1
    }
}

impl<'a, N, K> NodeKeyRef<'a, N, K> for (&'a N, &'a K)
where
    N: 'a,
    K: 'a,
{
    fn node(&self) -> &'a N {
        self.0
    }
    fn key(&self) -> &'a K {
        self.1
    }
}
